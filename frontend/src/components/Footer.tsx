import React from 'react';
import { FaFacebook, FaMedium, FaGithub } from 'react-icons/fa';
import './Footer.scss';

const Footer: React.FC = () => {
    return (
        <footer className="footer neumorphic">
            <div className="footer-content">
                <div className="footer-text">
                    © {new Date().getFullYear()} <a href="https://github.com/feyfry" target='_blank'>@feyfry</a>. All rights reserved.
                </div>
                <div className="social-links">
                    <a href="https://www.facebook.com/feyfry35" target='_blank' className="neumorphic-icon">
                        <FaFacebook />
                    </a>
                    <a href="https://feyfry.medium.com" target='_blank' className="neumorphic-icon">
                        <FaMedium />
                    </a>
                    <a href="https://github.com/feyfry?tab=repositories" target='_blank' className="neumorphic-icon">
                        <FaGithub />
                    </a>
                </div>
            </div>
        </footer>
    );
};

export default Footer;