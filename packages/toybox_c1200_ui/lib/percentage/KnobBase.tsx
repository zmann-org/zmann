'use client';
import { useId, useState } from 'react';
import {
  KnobHeadless,
  KnobHeadlessLabel,
  KnobHeadlessOutput,
} from 'react-knob-headless';
import { mapFrom01Linear, mapTo01Linear } from '@dsp-ts/math';
import { KnobBaseThumb } from '../KnobBaseThumb';
import useTheme from '@himalaya-ui/core/esm/use-theme';

type KnobHeadlessProps = React.ComponentProps<typeof KnobHeadless>;
type KnobBaseThumbProps = React.ComponentProps<typeof KnobBaseThumb>;
type KnobBaseProps = Pick<
  KnobHeadlessProps,
  | 'orientation'
  | 'mapTo01'
  | 'mapFrom01'
> &
  Pick<KnobBaseThumbProps, 'value01'> & {
    readonly label: string;
    readonly onValueRawChange: (value: number) => void; // Add this prop
  };

export function KnobBase({
  label,
  orientation,
  mapTo01 = mapTo01Linear,
  mapFrom01 = mapFrom01Linear,
  onValueRawChange, // Add this prop
}: KnobBaseProps) {
  const knobId = useId();
  const UITheme = useTheme();
  const labelId = useId();
  const [valueRaw, setValueRaw] = useState<number>(valueDefault);
  const value01 = mapTo01(valueRaw, valueMin, valueMax);
  const dragSensitivity = 0.006;
  return (
    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}
    >
      <KnobHeadlessLabel id={labelId} style={{ fontSize: 12 }}>{label}</KnobHeadlessLabel>
      <KnobHeadless
        style={{
          outline: "2px solid transparent",
          outlineOffset: 2,
          width: "4rem",
          height: "4rem",
          position: "relative",
          background:
            "linear-gradient(white, white) padding-box, linear-gradient(to bottom," +
            UITheme.palette.accents_5 +
            "B5," +
            UITheme.palette.background +
            ") border-box",
          borderRadius: "56em",
          border: "3px solid transparent",
        }}
        id={knobId}
        aria-labelledby={labelId}
        valueMin={valueMin}
        valueMax={valueMax}
        valueRaw={valueRaw}
        valueRawRoundFn={valueRawRoundFn}
        valueRawDisplayFn={valueRawDisplayFn}
        dragSensitivity={dragSensitivity}
        orientation={orientation}
        mapTo01={mapTo01}
        mapFrom01={mapFrom01}
        onValueRawChange={(value) => { onValueRawChange(value); setValueRaw(value) }} // Pass the prop
      >
        <KnobBaseThumb value01={value01} />
      </KnobHeadless>
      <KnobHeadlessOutput htmlFor={knobId} style={{ fontSize: 12 }}>
        {valueRawDisplayFn(valueRaw)}
      </KnobHeadlessOutput>
    </div>
  );
}

const valueMin = 0;
const valueMax = 100;
const valueDefault = 50;
const valueRawRoundFn = Math.round;
const valueRawDisplayFn = (valueRaw: number): string =>
  `${valueRawRoundFn(valueRaw)}%`;