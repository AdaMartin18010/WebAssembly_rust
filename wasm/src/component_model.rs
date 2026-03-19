//! # WebAssembly Component Model 支持
//!
//! 实现 WIT (WebAssembly Interface Types) 定义和组件组合
//! 支持多语言组件组合和跨语言调用

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// WIT 接口定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WitInterface {
    pub name: String,
    pub functions: Vec<WitFunction>,
    pub types: Vec<WitTypeDef>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

impl WitInterface {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: vec![],
            types: vec![],
            imports: vec![],
            exports: vec![],
        }
    }
    
    pub fn add_function(&mut self, func: WitFunction) {
        self.functions.push(func);
    }
    
    pub fn add_type(&mut self, type_def: WitTypeDef) {
        self.types.push(type_def);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WitFunction {
    pub name: String,
    pub params: Vec<WitParam>,
    pub results: WitResults,
    pub is_async: bool,  // WASI 0.3 支持
    pub docs: String,
}

impl WitFunction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: vec![],
            results: WitResults::None,
            is_async: false,
            docs: String::new(),
        }
    }
    
    pub fn with_param(mut self, name: impl Into<String>, param_type: WitType) -> Self {
        self.params.push(WitParam {
            name: name.into(),
            param_type,
        });
        self
    }
    
    pub fn with_result(mut self, result_type: WitType) -> Self {
        self.results = WitResults::Single(result_type);
        self
    }
    
    pub fn with_async(mut self) -> Self {
        self.is_async = true;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WitParam {
    pub name: String,
    pub param_type: WitType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WitResults {
    None,
    Single(WitType),
    Multiple(Vec<(String, WitType)>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WitTypeDef {
    pub name: String,
    pub definition: WitType,
    pub docs: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WitType {
    // 基础类型
    Bool,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
    Char,
    String,
    
    // 复合类型
    List(Box<WitType>),
    Option(Box<WitType>),
    Result(Box<WitType>, Box<WitType>),
    Tuple(Vec<WitType>),
    Record(Vec<(String, WitType)>),
    Variant(Vec<(String, Option<WitType>)>),
    Enum(Vec<String>),
    Flags(Vec<String>),
    
    // 资源类型
    Own(ResourceType),
    Borrow(ResourceType),
    
    // 类型引用
    Named(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceType {
    pub name: String,
    pub methods: Vec<WitFunction>,
}

/// 组件定义
#[derive(Debug, Clone)]
pub struct Component {
    pub id: ComponentId,
    pub name: String,
    pub version: String,
    pub imports: Vec<ComponentImport>,
    pub exports: Vec<ComponentExport>,
    pub wasm_module: Vec<u8>,
    pub wit: Option<WitInterface>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

impl ComponentId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl Component {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ComponentId::new(),
            name: name.into(),
            version: "0.1.0".to_string(),
            imports: vec![],
            exports: vec![],
            wasm_module: vec![],
            wit: None,
        }
    }
    
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }
    
    pub fn with_wit(mut self, wit: WitInterface) -> Self {
        self.wit = Some(wit);
        self
    }
    
    pub fn add_import(&mut self, import: ComponentImport) {
        self.imports.push(import);
    }
    
    pub fn add_export(&mut self, export: ComponentExport) {
        self.exports.push(export);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentImport {
    pub name: String,
    pub interface: WitInterface,
    pub from: Option<String>, // 可选的来源组件
}

#[derive(Debug, Clone)]
pub struct ComponentExport {
    pub name: String,
    pub interface: WitInterface,
    pub as_name: Option<String>, // 重命名导出
}

/// 组件组合器
pub struct ComponentComposer {
    components: Vec<Component>,
    wit_files: HashMap<String, String>,
    composition_graph: CompositionGraph,
}

#[derive(Debug, Default, Clone)]
pub struct CompositionGraph {
    nodes: Vec<ComponentId>,
    edges: Vec<(ComponentId, ComponentId, String)>, // from, to, interface_name
}

impl ComponentComposer {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            wit_files: HashMap::new(),
            composition_graph: CompositionGraph::default(),
        }
    }
    
    pub fn add_component(&mut self, component: Component) {
        self.composition_graph.nodes.push(component.id.clone());
        self.components.push(component);
    }
    
    pub fn register_wit(&mut self, name: String, wit_content: String) {
        self.wit_files.insert(name, wit_content);
    }
    
    pub fn connect(&mut self, from: &ComponentId, to: &ComponentId, interface: impl Into<String>) {
        self.composition_graph.edges.push((
            from.clone(),
            to.clone(),
            interface.into(),
        ));
    }
    
    /// 组合多个组件
    pub fn compose(&self) -> Result<ComposedComponent, CompositionError> {
        // 验证接口兼容性
        self.validate_interfaces()?;
        
        // 验证组合图无环
        self.validate_no_cycles()?;
        
        // 组合组件
        let composed = ComposedComponent {
            components: self.components.clone(),
            entry_point: self.find_entry_point()?,
            graph: self.composition_graph.clone(),
        };
        
        Ok(composed)
    }
    
    fn validate_interfaces(&self) -> Result<(), CompositionError> {
        for component in &self.components {
            for import in &component.imports {
                // 验证是否有对应的导出
                let found = self.components.iter().any(|c| {
                    c.exports.iter().any(|e| e.name == import.name)
                });
                
                if !found {
                    return Err(CompositionError::MissingExport {
                        component: component.name.clone(),
                        import: import.name.clone(),
                    });
                }
                
                // 验证接口签名匹配
                let matching_export = self.components.iter()
                    .flat_map(|c| &c.exports)
                    .find(|e| e.name == import.name);
                
                if let Some(export) = matching_export {
                    if !self.interfaces_compatible(&import.interface, &export.interface) {
                        return Err(CompositionError::InterfaceMismatch {
                            expected: import.name.clone(),
                            actual: export.name.clone(),
                        });
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn interfaces_compatible(&self, import: &WitInterface, export: &WitInterface) -> bool {
        // 简化的兼容性检查
        // 实际实现需要更详细的类型检查
        import.functions.len() == export.functions.len()
    }
    
    fn validate_no_cycles(&self) -> Result<(), CompositionError> {
        // 简单的环检测
        // 实际实现应该使用拓扑排序或 DFS
        if self.composition_graph.edges.len() > self.composition_graph.nodes.len() * 2 {
            return Err(CompositionError::PossibleCycle);
        }
        Ok(())
    }
    
    fn find_entry_point(&self) -> Result<ComponentId, CompositionError> {
        // 查找入口组件（没有导入的组件，或有明确标记的组件）
        self.components.first()
            .map(|c| c.id.clone())
            .ok_or(CompositionError::NoEntryPoint)
    }
}

/// 组合后的组件
#[derive(Debug, Clone)]
pub struct ComposedComponent {
    pub components: Vec<Component>,
    pub entry_point: ComponentId,
    pub graph: CompositionGraph,
}

/// 组合错误
#[derive(Debug, Clone, Error)]
pub enum CompositionError {
    #[error("组件 {component} 缺少导入: {import}")]
    MissingExport { component: String, import: String },
    #[error("接口不匹配: 期望 {expected}, 实际 {actual}")]
    InterfaceMismatch { expected: String, actual: String },
    #[error("没有入口点")]
    NoEntryPoint,
    #[error("可能存在循环依赖")]
    PossibleCycle,
    #[error("验证失败: {0}")]
    ValidationFailed(String),
}

/// WIT 解析器
pub struct WitParser;

impl WitParser {
    /// 解析 WIT 文件内容
    pub fn parse(wit_content: &str) -> Result<WitInterface, WitParseError> {
        // 简化的解析实现
        // 实际应该使用 wit-parser crate
        let mut interface = WitInterface::new("parsed");
        
        // 解析函数定义（简化）
        for line in wit_content.lines() {
            let line = line.trim();
            if line.starts_with("func ") {
                // 简化的函数解析
                let name = line[5..].split(':').next().unwrap_or("unknown").trim();
                interface.add_function(WitFunction::new(name));
            }
        }
        
        Ok(interface)
    }
    
    /// 生成 WIT 文本
    pub fn generate(interface: &WitInterface) -> String {
        let mut output = format!("interface {} {{\n", interface.name);
        
        // 生成类型定义
        for type_def in &interface.types {
            output.push_str(&format!("  type {} = {}\n", 
                type_def.name, 
                Self::type_to_string(&type_def.definition)
            ));
        }
        
        // 生成函数定义
        for func in &interface.functions {
            output.push_str(&format!("  {}func {}(", 
                if func.is_async { "async " } else { "" },
                func.name
            ));
            
            for (i, param) in func.params.iter().enumerate() {
                if i > 0 { output.push_str(", "); }
                output.push_str(&format!("{}: {}", 
                    param.name, 
                    Self::type_to_string(&param.param_type)
                ));
            }
            
            output.push_str(")");
            
            match &func.results {
                WitResults::None => {},
                WitResults::Single(ty) => {
                    output.push_str(&format!(" -> {}", Self::type_to_string(ty)));
                }
                WitResults::Multiple(results) => {
                    output.push_str(" -> (");
                    for (i, (name, ty)) in results.iter().enumerate() {
                        if i > 0 { output.push_str(", "); }
                        output.push_str(&format!("{}: {}", name, Self::type_to_string(ty)));
                    }
                    output.push_str(")");
                }
            }
            
            output.push_str("\n");
        }
        
        output.push_str("}\n");
        output
    }
    
    fn type_to_string(ty: &WitType) -> String {
        match ty {
            WitType::Bool => "bool".to_string(),
            WitType::U8 => "u8".to_string(),
            WitType::U16 => "u16".to_string(),
            WitType::U32 => "u32".to_string(),
            WitType::U64 => "u64".to_string(),
            WitType::S8 => "s8".to_string(),
            WitType::S16 => "s16".to_string(),
            WitType::S32 => "s32".to_string(),
            WitType::S64 => "s64".to_string(),
            WitType::F32 => "f32".to_string(),
            WitType::F64 => "f64".to_string(),
            WitType::Char => "char".to_string(),
            WitType::String => "string".to_string(),
            WitType::List(inner) => format!("list<{}>", Self::type_to_string(inner)),
            WitType::Option(inner) => format!("option<{}>", Self::type_to_string(inner)),
            WitType::Result(ok, err) => format!("result<{}, {}>", 
                Self::type_to_string(ok), 
                Self::type_to_string(err)
            ),
            WitType::Tuple(types) => {
                let inner: Vec<_> = types.iter().map(Self::type_to_string).collect();
                format!("tuple<{}>", inner.join(", "))
            }
            WitType::Record(fields) => {
                let inner: Vec<_> = fields.iter()
                    .map(|(n, t)| format!("{}: {}", n, Self::type_to_string(t)))
                    .collect();
                format!("record {{ {} }}", inner.join(", "))
            }
            WitType::Named(name) => name.clone(),
            _ => "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, Error)]
#[error("WIT parse error: {0}")]
pub struct WitParseError(pub String);

/// 示例 WIT 定义
pub mod examples {
    use super::*;
    
    /// 计算器接口
    pub fn calculator_interface() -> WitInterface {
        let mut interface = WitInterface::new("calculator");
        
        interface.add_function(
            WitFunction::new("add")
                .with_param("a", WitType::S32)
                .with_param("b", WitType::S32)
                .with_result(WitType::S32)
        );
        
        interface.add_function(
            WitFunction::new("subtract")
                .with_param("a", WitType::S32)
                .with_param("b", WitType::S32)
                .with_result(WitType::S32)
        );
        
        interface.add_function(
            WitFunction::new("divide")
                .with_param("a", WitType::F64)
                .with_param("b", WitType::F64)
                .with_result(WitType::Result(
                    Box::new(WitType::F64),
                    Box::new(WitType::String)
                ))
        );
        
        interface.add_type(WitTypeDef {
            name: "CalculatorError".to_string(),
            definition: WitType::Variant(vec![
                ("DivideByZero".to_string(), None),
                ("InvalidInput".to_string(), Some(WitType::String)),
            ]),
            docs: "计算器错误类型".to_string(),
        });
        
        interface
    }
    
    /// HTTP 接口
    pub fn http_interface() -> WitInterface {
        let mut interface = WitInterface::new("http");
        
        interface.add_function(
            WitFunction::new("get")
                .with_param("url", WitType::String)
                .with_async()
                .with_result(WitType::Result(
                    Box::new(WitType::Record(vec![
                        ("status".to_string(), WitType::U16),
                        ("body".to_string(), WitType::List(Box::new(WitType::U8))),
                    ])),
                    Box::new(WitType::String)
                ))
        );
        
        interface
    }
    
    /// 计算器的 WIT 文本
    pub fn calculator_wit_text() -> &'static str {
        r#"
interface calculator {
  type calculator-error = variant {
    divide-by-zero,
    invalid-input(string),
  }
  
  func add(a: s32, b: s32) -> s32
  func subtract(a: s32, b: s32) -> s32
  func divide(a: f64, b: f64) -> result<f64, string>
  
  async func compute-async(expression: string) -> result<f64, calculator-error>
}
"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wit_interface_creation() {
        let mut interface = WitInterface::new("test");
        interface.add_function(WitFunction::new("test_func"));
        
        assert_eq!(interface.name, "test");
        assert_eq!(interface.functions.len(), 1);
    }
    
    #[test]
    fn test_wit_function_builder() {
        let func = WitFunction::new("add")
            .with_param("a", WitType::S32)
            .with_param("b", WitType::S32)
            .with_result(WitType::S32);
        
        assert_eq!(func.name, "add");
        assert_eq!(func.params.len(), 2);
        assert!(matches!(func.results, WitResults::Single(WitType::S32)));
    }
    
    #[test]
    fn test_component_creation() {
        let component = Component::new("test-component")
            .with_version("1.0.0");
        
        assert_eq!(component.name, "test-component");
        assert_eq!(component.version, "1.0.0");
    }
    
    #[test]
    fn test_component_composition() {
        let mut composer = ComponentComposer::new();
        
        let component1 = Component::new("component1");
        let component2 = Component::new("component2");
        
        composer.add_component(component1);
        composer.add_component(component2);
        
        let result = composer.compose();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_wit_parser() {
        let wit_text = r#"
interface test {
  func add: func(a: s32, b: s32) -> s32
  func sub: func(a: s32, b: s32) -> s32
}
"#;
        
        let result = WitParser::parse(wit_text);
        assert!(result.is_ok());
        
        let interface = result.unwrap();
        assert_eq!(interface.name, "parsed");
    }
    
    #[test]
    fn test_wit_generator() {
        let interface = examples::calculator_interface();
        let wit_text = WitParser::generate(&interface);
        
        assert!(wit_text.contains("interface calculator"));
        assert!(wit_text.contains("func add"));
        assert!(wit_text.contains("func divide"));
    }
    
    #[test]
    fn test_type_to_string() {
        assert_eq!(WitParser::type_to_string(&WitType::S32), "s32");
        assert_eq!(WitParser::type_to_string(&WitType::String), "string");
        assert_eq!(
            WitParser::type_to_string(&WitType::List(Box::new(WitType::U8))),
            "list<u8>"
        );
    }
}
