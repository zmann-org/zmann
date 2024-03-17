import React from "react";

interface HeaderProps {
  children: React.ReactNode;
}

const Header: React.FC<HeaderProps> = ({ children }) => {
  return (
    <>
      <nav>{children}</nav>
      <style jsx>{`
        nav {
          background-color: #282729;
          border-bottom: 1px solid #515151;
          height: 48px;
          display: grid;
          grid-template-columns: auto 1fr auto;
          align-items: center;
          z-index: 100;
        }
      `}</style>
    </>
  );
};

export default Header;
