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
        }
      `}</style>
      <style jsx global>{`
      .slider .dot {
        min-height: unset !important;
        min-width: unset !important;
        height: 16px !important;
        width: 2px !important;
      }
      `}</style>
    </>
  );
};

export default HeaderRight;
