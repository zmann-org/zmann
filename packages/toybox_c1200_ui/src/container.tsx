import React from "react";

interface ContainerProps {
  children: React.ReactNode;
}

const Container: React.FC<ContainerProps> = ({ children }) => {
  return (
    <>
      <main>{children}</main>
      <style jsx>{`
        main {
          height: calc(100% - 48px);
          width: 100%;
          padding-bottom: 0;
        }
      `}</style>
      <style jsx global>{`
        .card {
          background: rgba(40, 39, 41, 0.698) !important;
          border: 1px solid #333 !important;
        }
        .module-name {
          background-color: #282729 !important;
          border: 1px solid #333 !important;
        }
        .content {
          color: #fff !important;
        }
        main {
          background: #121113 !important;
        }
      `}</style>
    </>
  );
};

export default Container;
