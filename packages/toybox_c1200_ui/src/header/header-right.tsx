import React from "react";

interface HeaderRightProps {
  children: React.ReactNode;
}

const HeaderRight: React.FC<HeaderRightProps> = ({ children }) => {
  return (
    <>
      <div className="header-right">{children}</div>
      <style jsx>{`
        .header-right {
          width: 100%;
          justify-self: end;
          display: flex;
          align-items: center;
          gap: 5px;
          margin-right: 10px;
        }
      `}</style>
    </>
  );
};

export default HeaderRight;
