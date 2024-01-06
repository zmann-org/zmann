import { mapFrom01Linear } from "@dsp-ts/math";
import { useTheme } from "@himalaya-ui/core";

type KnobBaseThumbProps = {
  readonly value01: number;
};

export function KnobBaseThumb({ value01 }: KnobBaseThumbProps) {
  const UItheme = useTheme();
  const angleMin = -145;
  const angleMax = 145;
  const angle = mapFrom01Linear(value01, angleMin, angleMax);
  return (
    <div
      style={{
        borderRadius: 9999,
        width: "100%",
        height: "100%",
        position: "absolute",
        backgroundColor: UItheme.palette.accents_2,
      }}
    >
      <div
        style={{
          width: "100%",
          height: "100%",
          position: "absolute",
          rotate: `${angle}deg`,
        }}
      >
        <div
          style={{
            borderRadius: "0.125rem",
            width: 2,
            height: "50%",
            top: 0,
            left: "50%",
            backgroundColor: UItheme.palette.accents_0,
            position: "absolute",
            transform:
              "translateX(-50%) rotate(0) skewX(0) skewY(0) scaleX(1) scaleY(1)",
          }}
        />
      </div>
    </div>
  );
}
