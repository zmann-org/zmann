import React from "react";

interface ViewProps {
  children: React.ReactNode;
}

const View: React.FC<ViewProps> = ({ children }) => {
  return (
    <>
      <div className="view">{children}</div>
      <style jsx>{`
        height: 100vh;
        display: flex;
        flex-direction: column;
      `}</style>
    </>
  );
};

export default View;
