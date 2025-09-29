// WebAssembly 2.0 + Rust 1.90 语言选择器组件
// 多语言切换界面

import i18n from '../i18n/index.js';

class LanguageSelector {
    constructor(container) {
        this.container = container;
        this.currentLanguage = i18n.getCurrentLanguage();
        this.availableLanguages = i18n.getAvailableLanguages();
        this.isOpen = false;
        
        this.init();
    }
    
    init() {
        this.render();
        this.bindEvents();
    }
    
    render() {
        this.container.innerHTML = `
            <div class="language-selector">
                <button class="language-toggle" aria-haspopup="true" aria-expanded="false">
                    <span class="language-flag">${this.currentLanguage.flag}</span>
                    <span class="language-name">${this.currentLanguage.name}</span>
                    <span class="language-arrow" aria-hidden="true">▼</span>
                </button>
                <div class="language-dropdown" role="menu" aria-hidden="true">
                    ${this.renderLanguageOptions()}
                </div>
            </div>
        `;
    }
    
    renderLanguageOptions() {
        return this.availableLanguages.map(lang => `
            <button class="language-option ${lang.code === this.currentLanguage.code ? 'active' : ''}" 
                    data-language="${lang.code}" 
                    role="menuitem"
                    aria-label="${i18n.t('language.select')}: ${lang.name}">
                <span class="language-flag">${lang.flag}</span>
                <span class="language-name">${lang.name}</span>
                ${lang.code === this.currentLanguage.code ? '<span class="checkmark" aria-hidden="true">✓</span>' : ''}
            </button>
        `).join('');
    }
    
    bindEvents() {
        const toggle = this.container.querySelector('.language-toggle');
        const dropdown = this.container.querySelector('.language-dropdown');
        const options = this.container.querySelectorAll('.language-option');
        
        // 切换下拉菜单
        toggle.addEventListener('click', (e) => {
            e.stopPropagation();
            this.toggleDropdown();
        });
        
        // 点击外部关闭下拉菜单
        document.addEventListener('click', (e) => {
            if (!this.container.contains(e.target)) {
                this.closeDropdown();
            }
        });
        
        // 键盘导航
        toggle.addEventListener('keydown', (e) => {
            switch (e.key) {
                case 'Enter':
                case ' ':
                    e.preventDefault();
                    this.toggleDropdown();
                    break;
                case 'ArrowDown':
                    e.preventDefault();
                    this.openDropdown();
                    this.focusFirstOption();
                    break;
                case 'Escape':
                    this.closeDropdown();
                    break;
            }
        });
        
        // 语言选项点击事件
        options.forEach(option => {
            option.addEventListener('click', (e) => {
                e.preventDefault();
                const languageCode = option.dataset.language;
                this.selectLanguage(languageCode);
            });
            
            // 键盘导航
            option.addEventListener('keydown', (e) => {
                switch (e.key) {
                    case 'Enter':
                    case ' ':
                        e.preventDefault();
                        const languageCode = option.dataset.language;
                        this.selectLanguage(languageCode);
                        break;
                    case 'ArrowDown':
                        e.preventDefault();
                        this.focusNextOption(option);
                        break;
                    case 'ArrowUp':
                        e.preventDefault();
                        this.focusPreviousOption(option);
                        break;
                    case 'Escape':
                        this.closeDropdown();
                        toggle.focus();
                        break;
                }
            });
        });
    }
    
    toggleDropdown() {
        if (this.isOpen) {
            this.closeDropdown();
        } else {
            this.openDropdown();
        }
    }
    
    openDropdown() {
        const toggle = this.container.querySelector('.language-toggle');
        const dropdown = this.container.querySelector('.language-dropdown');
        
        this.isOpen = true;
        toggle.setAttribute('aria-expanded', 'true');
        dropdown.setAttribute('aria-hidden', 'false');
        dropdown.classList.add('open');
        
        // 添加动画效果
        requestAnimationFrame(() => {
            dropdown.style.transform = 'translateY(0)';
            dropdown.style.opacity = '1';
        });
    }
    
    closeDropdown() {
        const toggle = this.container.querySelector('.language-toggle');
        const dropdown = this.container.querySelector('.language-dropdown');
        
        this.isOpen = false;
        toggle.setAttribute('aria-expanded', 'false');
        dropdown.setAttribute('aria-hidden', 'true');
        dropdown.classList.remove('open');
        
        // 添加动画效果
        dropdown.style.transform = 'translateY(-10px)';
        dropdown.style.opacity = '0';
    }
    
    selectLanguage(languageCode) {
        if (i18n.setLanguage(languageCode)) {
            this.currentLanguage = i18n.getCurrentLanguage();
            this.render();
            this.bindEvents();
            this.closeDropdown();
            
            // 触发语言变更事件
            this.container.dispatchEvent(new CustomEvent('languageChanged', {
                detail: { language: languageCode }
            }));
            
            // 显示成功消息
            this.showSuccessMessage(i18n.t('success.languageChanged'));
        }
    }
    
    focusFirstOption() {
        const firstOption = this.container.querySelector('.language-option');
        if (firstOption) {
            firstOption.focus();
        }
    }
    
    focusNextOption(currentOption) {
        const options = Array.from(this.container.querySelectorAll('.language-option'));
        const currentIndex = options.indexOf(currentOption);
        const nextIndex = (currentIndex + 1) % options.length;
        options[nextIndex].focus();
    }
    
    focusPreviousOption(currentOption) {
        const options = Array.from(this.container.querySelectorAll('.language-option'));
        const currentIndex = options.indexOf(currentOption);
        const previousIndex = currentIndex === 0 ? options.length - 1 : currentIndex - 1;
        options[previousIndex].focus();
    }
    
    showSuccessMessage(message) {
        // 创建临时成功消息
        const successMessage = document.createElement('div');
        successMessage.className = 'language-success-message';
        successMessage.textContent = message;
        successMessage.setAttribute('role', 'status');
        successMessage.setAttribute('aria-live', 'polite');
        
        // 添加到页面
        document.body.appendChild(successMessage);
        
        // 显示动画
        requestAnimationFrame(() => {
            successMessage.classList.add('show');
        });
        
        // 3秒后移除
        setTimeout(() => {
            successMessage.classList.remove('show');
            setTimeout(() => {
                if (successMessage.parentNode) {
                    successMessage.parentNode.removeChild(successMessage);
                }
            }, 300);
        }, 3000);
    }
    
    // 获取当前语言
    getCurrentLanguage() {
        return this.currentLanguage;
    }
    
    // 更新语言选择器
    update() {
        this.currentLanguage = i18n.getCurrentLanguage();
        this.render();
        this.bindEvents();
    }
}

// 添加语言选择器样式
const languageSelectorStyles = `
<style>
.language-selector {
    position: relative;
    display: inline-block;
}

.language-toggle {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: white;
    border: 2px solid var(--gray-200);
    border-radius: var(--radius-lg);
    cursor: pointer;
    transition: var(--transition-fast);
    font-size: 14px;
    font-weight: 500;
    color: var(--gray-700);
}

.language-toggle:hover {
    border-color: var(--primary-color);
    background: var(--gray-50);
}

.language-toggle:focus {
    outline: 2px solid var(--primary-color);
    outline-offset: 2px;
}

.language-toggle[aria-expanded="true"] {
    border-color: var(--primary-color);
    background: var(--primary-color);
    color: white;
}

.language-toggle[aria-expanded="true"] .language-arrow {
    transform: rotate(180deg);
}

.language-flag {
    font-size: 16px;
}

.language-name {
    min-width: 80px;
    text-align: left;
}

.language-arrow {
    transition: transform 0.2s ease;
    font-size: 12px;
}

.language-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: white;
    border: 2px solid var(--gray-200);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    z-index: 1000;
    max-height: 300px;
    overflow-y: auto;
    transform: translateY(-10px);
    opacity: 0;
    transition: all 0.2s ease;
    margin-top: 4px;
}

.language-dropdown.open {
    transform: translateY(0);
    opacity: 1;
}

.language-option {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 12px;
    background: none;
    border: none;
    cursor: pointer;
    transition: var(--transition-fast);
    font-size: 14px;
    color: var(--gray-700);
    text-align: left;
}

.language-option:hover {
    background: var(--gray-50);
}

.language-option:focus {
    outline: 2px solid var(--primary-color);
    outline-offset: -2px;
    background: var(--gray-50);
}

.language-option.active {
    background: var(--primary-color);
    color: white;
}

.language-option.active:hover {
    background: var(--primary-color);
    opacity: 0.9;
}

.checkmark {
    margin-left: auto;
    font-weight: bold;
}

.language-success-message {
    position: fixed;
    top: 20px;
    right: 20px;
    background: var(--success-color);
    color: white;
    padding: 12px 20px;
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    z-index: 10000;
    transform: translateX(100%);
    transition: transform 0.3s ease;
}

.language-success-message.show {
    transform: translateX(0);
}

/* 响应式设计 */
@media (max-width: 768px) {
    .language-selector {
        width: 100%;
    }
    
    .language-toggle {
        width: 100%;
        justify-content: center;
    }
    
    .language-dropdown {
        left: 0;
        right: 0;
    }
}

/* 深色模式支持 */
@media (prefers-color-scheme: dark) {
    .language-toggle {
        background: var(--gray-800);
        border-color: var(--gray-600);
        color: var(--gray-200);
    }
    
    .language-toggle:hover {
        background: var(--gray-700);
    }
    
    .language-dropdown {
        background: var(--gray-800);
        border-color: var(--gray-600);
    }
    
    .language-option {
        color: var(--gray-200);
    }
    
    .language-option:hover {
        background: var(--gray-700);
    }
}

/* 高对比度模式 */
@media (prefers-contrast: high) {
    .language-toggle {
        border-width: 3px;
    }
    
    .language-dropdown {
        border-width: 3px;
    }
    
    .language-option:focus {
        outline-width: 3px;
    }
}

/* 减少动画偏好 */
@media (prefers-reduced-motion: reduce) {
    .language-toggle,
    .language-dropdown,
    .language-option,
    .language-success-message {
        transition: none;
    }
    
    .language-arrow {
        transition: none;
    }
}
</style>
`;

// 将样式添加到页面
if (!document.querySelector('#language-selector-styles')) {
    const styleElement = document.createElement('div');
    styleElement.id = 'language-selector-styles';
    styleElement.innerHTML = languageSelectorStyles;
    document.head.appendChild(styleElement);
}

export default LanguageSelector;
