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
          align-items: center;
          justify-content: flex-end;
          justify-self: end;
          display: flex;
          align-items: center;
          gap: 5px;
          min-width: 124.34px;
          width: 210px;
          margin-left: calc(-1 * calc(210px - 123.34px));
        }
      `}</style>
    </>
  );
};

export default HeaderRight;
