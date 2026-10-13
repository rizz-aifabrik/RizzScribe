// Sidebar icon: a small voice waveform.
const HandyHand = ({
  width,
  height,
}: {
  width?: number | string;
  height?: number | string;
}) => (
  <svg
    width={width || 126}
    height={height || 135}
    viewBox="0 0 126 135"
    className="fill-text stroke-text"
    xmlns="http://www.w3.org/2000/svg"
  >
    {[
      [14, 52, 31],
      [34, 28, 79],
      [54, 8, 119],
      [74, 36, 71],
      [94, 56, 23],
    ].map(([x, h, y], i) => (
      <rect
        key={i}
        x={x}
        y={67 - h / 2 - 0 * y}
        width="16"
        height={h * 1.6}
        rx="8"
        transform={`translate(0 ${-h * 0.3})`}
        strokeWidth="0"
      />
    ))}
  </svg>
);

export default HandyHand;
