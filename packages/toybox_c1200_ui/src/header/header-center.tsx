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
          justify-self: center;
          min-width: 200px;
        }
      `}</style>
    </>
  );
};

export default HeaderCenter;
