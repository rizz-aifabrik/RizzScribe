import React from "react";

const HandyTextLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <svg
      width={width}
      height={height}
      className={className}
      viewBox="0 0 300 70"
      xmlns="http://www.w3.org/2000/svg"
    >
      <text
        x="0"
        y="52"
        fontFamily="-apple-system, 'SF Pro Display', Helvetica, Arial, sans-serif"
        fontSize="54"
        fontWeight="700"
        fill="#F2488F"
      >
        RizzScribe
      </text>
    </svg>
  );
};

export default HandyTextLogo;
