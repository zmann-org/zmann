import React from "react";

interface ToyboxProps {
  size?: number;
  color?: string;
  style?: React.CSSProperties;
}

const Toybox: React.FC<ToyboxProps> = ({
  size = 24,
  color,
  style,
  ...props
}) => {
  return (
    <svg
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="1.5"
      shapeRendering="geometricPrecision"
      viewBox="0 0 24 24"
      {...props}
      height={size}
      width={size}
      style={{ ...style, color: color }}
    >
      <path
        d="M21 15.9999V7.9999C20.9996 7.64918 20.9071 7.30471 20.7315 7.00106C20.556 6.69742 20.3037 6.44526 20 6.2699L13 2.2699C12.696 2.09437 12.3511 2.00195 12 2.00195C11.6489 2.00195 11.304 2.09437 11 2.2699L4 6.2699C3.69626 6.44526 3.44398 6.69742 3.26846 7.00106C3.09294 7.30471 3.00036 7.64918 3 7.9999V15.9999C3.00036 16.3506 3.09294 16.6951 3.26846 16.9987C3.44398 17.3024 3.69626 17.5545 4 17.7299L11 21.7299C11.304 21.9054 11.6489 21.9979 12 21.9979C12.3511 21.9979 12.696 21.9054 13 21.7299L20 17.7299C20.3037 17.5545 20.556 17.3024 20.7315 16.9987C20.9071 16.6951 20.9996 16.3506 21 15.9999Z"
        stroke="currentColor"
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path
        d="M3.27002 6.95996L12 12.01L20.73 6.95996M12 22.08V12"
        stroke="currentColor"
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path
        d="M4.5 10L7.5 11.75M10.5 13.5L7.5 11.75M7.5 11.75V18"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path
        d="M14 15.875V12.75L17.5181 10.8151C18.1845 10.4485 19 10.9307 19 11.6913V12.75M14 15.875V19L18.5735 15.7986C18.8408 15.6114 19 15.3057 19 14.9793V12.75M14 15.875L19 12.75"
        stroke="currentColor"
        strokeLinejoin="round"
      />
    </svg>
  );
};
export default Toybox;
