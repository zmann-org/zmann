import React, { useState, useRef } from "react";

const CustomSlider = ({
  onChange,
  defaultValue,
}: {
  onChange: (value: number) => void;
  defaultValue: number;
}) => {
  const [value, setValue] = useState(defaultValue || 0);
  const inputRef = useRef(null);

  const handleInputChange = (event: { target: { value: string } }) => {
    const newValue = parseFloat(event.target.value);
    setValue(newValue);
    if (onChange) {
      onChange(newValue);
    }
  };

  return (
    <div>
      <input
        type="range"
        value={value}
        max={1}
        min={0}
        step={0.01}
        className="input-knob"
        data-src="https://i.imgur.com/K5NDNNK.png"
        data-sprites="78"
        onChange={handleInputChange}
        ref={inputRef}
      />
      test
      {/* You can add additional elements or styles for your slider */}
    </div>
  );
};

export { CustomSlider };
