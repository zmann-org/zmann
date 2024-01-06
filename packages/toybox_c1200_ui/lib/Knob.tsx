"use client";
import { useState } from "react";
import { KnobHeadless } from "react-knob-headless";
import { mapTo01Linear } from "@dsp-ts/math";
import { KnobBaseThumb } from "./KnobBaseThumb";

type KnobBaseThumbProps = React.ComponentProps<typeof KnobBaseThumb>;
type KnobDecorativeProps = Pick<KnobBaseThumbProps, "value01"> & {
  readonly valueDefault: number;
};

export function KnobDecorative({ valueDefault }: KnobDecorativeProps) {
  const [valueRaw, setValueRaw] = useState<number>(valueDefault);
  const value01 = mapTo01Linear(valueRaw, valueMin, valueMax);
  return (
    <KnobHeadless
      style={{
        outline: "2px solid transparent",
        outlineOffset: 2,
        width: "4rem",
        height: "4rem",
        position: "relative",
      }}
      aria-label={""}
      valueMin={valueMin}
      valueMax={valueMax}
      dragSensitivity={dragSensitivity}
      valueRaw={valueRaw}
      valueRawRoundFn={valueRawRoundFn}
      valueRawDisplayFn={valueRawDisplayFn}
      onValueRawChange={setValueRaw}
    >
      <KnobBaseThumb value01={value01} />
    </KnobHeadless>
  );
}

const valueMin = 0;
const valueMax = 100;
const dragSensitivity = 0.006;
const valueRawRoundFn = Math.round;
const valueRawDisplayFn = (valueRaw: number): string =>
  `${valueRawRoundFn(valueRaw)} units`;
