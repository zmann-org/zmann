import React from "react";

interface HeaderLeftProps {
  children: React.ReactNode;
}

const HeaderLeft: React.FC<HeaderLeftProps> = ({ children }) => {
  return (
    <>
      <div className="header-left">{children}</div>
      <style jsx>{`
        .header-left {
          display: flex;
          align-items: center;
          justify-content: flex-start;
          display: flex;
          vertical-align: middle;
          gap: 5px;
        }
      `}</style>
    </>
  );
};

export default HeaderLeft;
