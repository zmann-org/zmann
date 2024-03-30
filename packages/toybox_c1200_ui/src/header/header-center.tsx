import React from "react";

interface HeaderCenterProps {
  children: React.ReactNode;
}

const HeaderCenter: React.FC<HeaderCenterProps> = ({ children }) => {
  return (
    <>
      <div className="header-center">{children}</div>
      <style jsx>{`
        .header-center {
          display: flex;
          align-items: center;
          justify-content: center;
        }
      `}</style>
    </>
  );
};

export default HeaderCenter;
