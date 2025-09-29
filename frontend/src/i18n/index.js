// WebAssembly 2.0 + Rust 1.90 国际化支持
// 多语言支持和本地化

// 语言配置
const languages = {
    'zh-CN': {
        name: '简体中文',
        flag: '🇨🇳',
        direction: 'ltr'
    },
    'zh-TW': {
        name: '繁體中文',
        flag: '🇹🇼',
        direction: 'ltr'
    },
    'en-US': {
        name: 'English',
        flag: '🇺🇸',
        direction: 'ltr'
    },
    'ja-JP': {
        name: '日本語',
        flag: '🇯🇵',
        direction: 'ltr'
    },
    'ko-KR': {
        name: '한국어',
        flag: '🇰🇷',
        direction: 'ltr'
    },
    'fr-FR': {
        name: 'Français',
        flag: '🇫🇷',
        direction: 'ltr'
    },
    'de-DE': {
        name: 'Deutsch',
        flag: '🇩🇪',
        direction: 'ltr'
    },
    'es-ES': {
        name: 'Español',
        flag: '🇪🇸',
        direction: 'ltr'
    },
    'pt-BR': {
        name: 'Português',
        flag: '🇧🇷',
        direction: 'ltr'
    },
    'ru-RU': {
        name: 'Русский',
        flag: '🇷🇺',
        direction: 'ltr'
    }
};

// 翻译文本
const translations = {
    'zh-CN': {
        // 页面标题
        'page.title': 'WebAssembly 2.0 + Rust 1.90 演示',
        'page.description': '高性能WebAssembly应用演示平台',
        
        // 性能统计
        'performance.title': '📊 性能统计',
        'performance.loadTime': '加载时间 (ms)',
        'performance.memoryUsage': '内存使用 (MB)',
        'performance.executionTime': '执行时间 (ms)',
        'performance.throughput': '吞吐量 (ops/s)',
        
        // 演示功能
        'demo.basic.title': '🦀 基础示例',
        'demo.basic.description': '演示Rust 1.90新特性和基本的WebAssembly功能',
        'demo.basic.input.label': '斐波那契数列计算',
        'demo.basic.input.help': '输入1到50之间的数字来计算斐波那契数列',
        'demo.basic.button': '运行基础示例',
        'demo.basic.loading': '计算中...',
        
        'demo.advanced.title': '🌐 高级示例',
        'demo.advanced.description': '展示WebAssembly 2.0新特性和复杂计算',
        'demo.advanced.matrixSize': '矩阵大小',
        'demo.advanced.iterations': '迭代次数',
        'demo.advanced.button': '运行高级示例',
        'demo.advanced.loading': '处理中...',
        
        'demo.performance.title': '⚡ 性能示例',
        'demo.performance.description': '测试WebAssembly性能优化和SIMD操作',
        'demo.performance.testType': '性能测试类型',
        'demo.performance.dataSize': '数据大小',
        'demo.performance.button': '运行性能测试',
        'demo.performance.loading': '测试中...',
        
        'demo.image.title': '🖼️ 图像处理',
        'demo.image.description': 'WebAssembly图像处理和滤镜效果',
        'demo.image.filterType': '滤镜类型',
        'demo.image.imageSize': '图像大小',
        'demo.image.button': '运行图像处理',
        'demo.image.loading': '处理中...',
        
        // 测试类型选项
        'testType.fibonacci': '斐波那契缓存',
        'testType.matrix': '矩阵乘法',
        'testType.sorting': '快速排序',
        'testType.simd': 'SIMD向量运算',
        
        // 滤镜类型选项
        'filter.grayscale': '灰度',
        'filter.blur': '模糊',
        'filter.sharpen': '锐化',
        'filter.edgeDetect': '边缘检测',
        
        // 页脚
        'footer.copyright': '© 2025 WebAssembly 2.0 + Rust 1.90 演示项目',
        'footer.description': '基于最新的WebAssembly和Rust技术构建',
        
        // 错误消息
        'error.wasmNotLoaded': 'WebAssembly模块未加载',
        'error.wasmInitFailed': 'WebAssembly模块初始化失败，请检查浏览器兼容性',
        'error.demoFailed': 'Demo执行失败',
        'error.unknown': '未知错误',
        
        // 成功消息
        'success.wasmLoaded': 'WebAssembly模块加载成功',
        'success.demoCompleted': 'Demo执行完成',
        
        // 语言选择
        'language.select': '选择语言',
        'language.current': '当前语言'
    },
    
    'en-US': {
        // Page titles
        'page.title': 'WebAssembly 2.0 + Rust 1.90 Demo',
        'page.description': 'High-performance WebAssembly application demo platform',
        
        // Performance stats
        'performance.title': '📊 Performance Statistics',
        'performance.loadTime': 'Load Time (ms)',
        'performance.memoryUsage': 'Memory Usage (MB)',
        'performance.executionTime': 'Execution Time (ms)',
        'performance.throughput': 'Throughput (ops/s)',
        
        // Demo features
        'demo.basic.title': '🦀 Basic Example',
        'demo.basic.description': 'Demonstrates Rust 1.90 new features and basic WebAssembly functionality',
        'demo.basic.input.label': 'Fibonacci Sequence Calculation',
        'demo.basic.input.help': 'Enter a number between 1 and 50 to calculate the Fibonacci sequence',
        'demo.basic.button': 'Run Basic Example',
        'demo.basic.loading': 'Calculating...',
        
        'demo.advanced.title': '🌐 Advanced Example',
        'demo.advanced.description': 'Shows WebAssembly 2.0 new features and complex calculations',
        'demo.advanced.matrixSize': 'Matrix Size',
        'demo.advanced.iterations': 'Iterations',
        'demo.advanced.button': 'Run Advanced Example',
        'demo.advanced.loading': 'Processing...',
        
        'demo.performance.title': '⚡ Performance Example',
        'demo.performance.description': 'Tests WebAssembly performance optimization and SIMD operations',
        'demo.performance.testType': 'Performance Test Type',
        'demo.performance.dataSize': 'Data Size',
        'demo.performance.button': 'Run Performance Test',
        'demo.performance.loading': 'Testing...',
        
        'demo.image.title': '🖼️ Image Processing',
        'demo.image.description': 'WebAssembly image processing and filter effects',
        'demo.image.filterType': 'Filter Type',
        'demo.image.imageSize': 'Image Size',
        'demo.image.button': 'Run Image Processing',
        'demo.image.loading': 'Processing...',
        
        // Test type options
        'testType.fibonacci': 'Fibonacci Cache',
        'testType.matrix': 'Matrix Multiplication',
        'testType.sorting': 'Quick Sort',
        'testType.simd': 'SIMD Vector Operations',
        
        // Filter type options
        'filter.grayscale': 'Grayscale',
        'filter.blur': 'Blur',
        'filter.sharpen': 'Sharpen',
        'filter.edgeDetect': 'Edge Detection',
        
        // Footer
        'footer.copyright': '© 2025 WebAssembly 2.0 + Rust 1.90 Demo Project',
        'footer.description': 'Built with the latest WebAssembly and Rust technologies',
        
        // Error messages
        'error.wasmNotLoaded': 'WebAssembly module not loaded',
        'error.wasmInitFailed': 'WebAssembly module initialization failed, please check browser compatibility',
        'error.demoFailed': 'Demo execution failed',
        'error.unknown': 'Unknown error',
        
        // Success messages
        'success.wasmLoaded': 'WebAssembly module loaded successfully',
        'success.demoCompleted': 'Demo execution completed',
        
        // Language selection
        'language.select': 'Select Language',
        'language.current': 'Current Language'
    },
    
    'ja-JP': {
        // ページタイトル
        'page.title': 'WebAssembly 2.0 + Rust 1.90 デモ',
        'page.description': '高性能WebAssemblyアプリケーションデモプラットフォーム',
        
        // パフォーマンス統計
        'performance.title': '📊 パフォーマンス統計',
        'performance.loadTime': '読み込み時間 (ms)',
        'performance.memoryUsage': 'メモリ使用量 (MB)',
        'performance.executionTime': '実行時間 (ms)',
        'performance.throughput': 'スループット (ops/s)',
        
        // デモ機能
        'demo.basic.title': '🦀 基本例',
        'demo.basic.description': 'Rust 1.90の新機能と基本的なWebAssembly機能をデモンストレーション',
        'demo.basic.input.label': 'フィボナッチ数列計算',
        'demo.basic.input.help': '1から50の間の数字を入力してフィボナッチ数列を計算',
        'demo.basic.button': '基本例を実行',
        'demo.basic.loading': '計算中...',
        
        'demo.advanced.title': '🌐 高度な例',
        'demo.advanced.description': 'WebAssembly 2.0の新機能と複雑な計算を表示',
        'demo.advanced.matrixSize': 'マトリックスサイズ',
        'demo.advanced.iterations': '反復回数',
        'demo.advanced.button': '高度な例を実行',
        'demo.advanced.loading': '処理中...',
        
        'demo.performance.title': '⚡ パフォーマンス例',
        'demo.performance.description': 'WebAssemblyパフォーマンス最適化とSIMD操作をテスト',
        'demo.performance.testType': 'パフォーマンステストタイプ',
        'demo.performance.dataSize': 'データサイズ',
        'demo.performance.button': 'パフォーマンステストを実行',
        'demo.performance.loading': 'テスト中...',
        
        'demo.image.title': '🖼️ 画像処理',
        'demo.image.description': 'WebAssembly画像処理とフィルター効果',
        'demo.image.filterType': 'フィルタータイプ',
        'demo.image.imageSize': '画像サイズ',
        'demo.image.button': '画像処理を実行',
        'demo.image.loading': '処理中...',
        
        // テストタイプオプション
        'testType.fibonacci': 'フィボナッチキャッシュ',
        'testType.matrix': 'マトリックス乗算',
        'testType.sorting': 'クイックソート',
        'testType.simd': 'SIMDベクトル操作',
        
        // フィルタータイプオプション
        'filter.grayscale': 'グレースケール',
        'filter.blur': 'ブラー',
        'filter.sharpen': 'シャープ',
        'filter.edgeDetect': 'エッジ検出',
        
        // フッター
        'footer.copyright': '© 2025 WebAssembly 2.0 + Rust 1.90 デモプロジェクト',
        'footer.description': '最新のWebAssemblyとRust技術で構築',
        
        // エラーメッセージ
        'error.wasmNotLoaded': 'WebAssemblyモジュールが読み込まれていません',
        'error.wasmInitFailed': 'WebAssemblyモジュールの初期化に失敗しました。ブラウザの互換性を確認してください',
        'error.demoFailed': 'デモの実行に失敗しました',
        'error.unknown': '不明なエラー',
        
        // 成功メッセージ
        'success.wasmLoaded': 'WebAssemblyモジュールが正常に読み込まれました',
        'success.demoCompleted': 'デモの実行が完了しました',
        
        // 言語選択
        'language.select': '言語を選択',
        'language.current': '現在の言語'
    }
};

// 国际化类
class I18n {
    constructor() {
        this.currentLanguage = this.detectLanguage();
        this.fallbackLanguage = 'en-US';
    }
    
    // 检测用户语言
    detectLanguage() {
        // 从localStorage获取保存的语言
        const savedLanguage = localStorage.getItem('preferred-language');
        if (savedLanguage && translations[savedLanguage]) {
            return savedLanguage;
        }
        
        // 从浏览器语言设置检测
        const browserLanguage = navigator.language || navigator.languages[0];
        if (translations[browserLanguage]) {
            return browserLanguage;
        }
        
        // 检测语言前缀匹配
        const languagePrefix = browserLanguage.split('-')[0];
        for (const lang in translations) {
            if (lang.startsWith(languagePrefix)) {
                return lang;
            }
        }
        
        // 默认返回英语
        return this.fallbackLanguage;
    }
    
    // 获取翻译文本
    t(key, params = {}) {
        const translation = translations[this.currentLanguage]?.[key] || 
                           translations[this.fallbackLanguage]?.[key] || 
                           key;
        
        // 替换参数
        return translation.replace(/\{\{(\w+)\}\}/g, (match, param) => {
            return params[param] || match;
        });
    }
    
    // 设置语言
    setLanguage(language) {
        if (translations[language]) {
            this.currentLanguage = language;
            localStorage.setItem('preferred-language', language);
            this.updatePageLanguage();
            this.updatePageContent();
            return true;
        }
        return false;
    }
    
    // 更新页面语言属性
    updatePageLanguage() {
        document.documentElement.lang = this.currentLanguage;
        document.documentElement.dir = languages[this.currentLanguage]?.direction || 'ltr';
    }
    
    // 更新页面内容
    updatePageContent() {
        // 更新所有带有data-i18n属性的元素
        document.querySelectorAll('[data-i18n]').forEach(element => {
            const key = element.getAttribute('data-i18n');
            const translation = this.t(key);
            
            if (element.tagName === 'INPUT' && element.type === 'text') {
                element.placeholder = translation;
            } else if (element.tagName === 'INPUT' && element.type === 'submit') {
                element.value = translation;
            } else {
                element.textContent = translation;
            }
        });
        
        // 更新所有带有data-i18n-attr属性的元素
        document.querySelectorAll('[data-i18n-attr]').forEach(element => {
            const attrConfig = element.getAttribute('data-i18n-attr');
            const [attr, key] = attrConfig.split(':');
            const translation = this.t(key);
            element.setAttribute(attr, translation);
        });
        
        // 更新页面标题
        document.title = this.t('page.title');
        
        // 更新meta描述
        const metaDescription = document.querySelector('meta[name="description"]');
        if (metaDescription) {
            metaDescription.content = this.t('page.description');
        }
    }
    
    // 获取可用语言列表
    getAvailableLanguages() {
        return Object.keys(languages).map(code => ({
            code,
            name: languages[code].name,
            flag: languages[code].flag
        }));
    }
    
    // 获取当前语言信息
    getCurrentLanguage() {
        return {
            code: this.currentLanguage,
            name: languages[this.currentLanguage]?.name,
            flag: languages[this.currentLanguage]?.flag
        };
    }
    
    // 格式化数字
    formatNumber(number, options = {}) {
        return new Intl.NumberFormat(this.currentLanguage, options).format(number);
    }
    
    // 格式化日期
    formatDate(date, options = {}) {
        return new Intl.DateTimeFormat(this.currentLanguage, options).format(date);
    }
    
    // 格式化时间
    formatTime(date, options = {}) {
        return new Intl.DateTimeFormat(this.currentLanguage, {
            ...options,
            timeStyle: 'medium'
        }).format(date);
    }
    
    // 格式化货币
    formatCurrency(amount, currency = 'USD', options = {}) {
        return new Intl.NumberFormat(this.currentLanguage, {
            style: 'currency',
            currency,
            ...options
        }).format(amount);
    }
    
    // 获取相对时间
    formatRelativeTime(date, options = {}) {
        const rtf = new Intl.RelativeTimeFormat(this.currentLanguage, options);
        const now = new Date();
        const diffInSeconds = Math.floor((date - now) / 1000);
        
        if (Math.abs(diffInSeconds) < 60) {
            return rtf.format(diffInSeconds, 'second');
        } else if (Math.abs(diffInSeconds) < 3600) {
            return rtf.format(Math.floor(diffInSeconds / 60), 'minute');
        } else if (Math.abs(diffInSeconds) < 86400) {
            return rtf.format(Math.floor(diffInSeconds / 3600), 'hour');
        } else {
            return rtf.format(Math.floor(diffInSeconds / 86400), 'day');
        }
    }
}

// 创建全局i18n实例
const i18n = new I18n();

// 导出
export { i18n, languages, translations };
export default i18n;
