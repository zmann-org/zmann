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
        filter: "drop-shadow(0px 0px 5px " + UItheme.palette.background + ")",
        background: "linear-gradient(to bottom, "+UItheme.palette.accents_2+", #2f2f2f)",
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
            marginTop: 4,
            borderRadius: "0.125rem",
            width: 4,
            height: "40%",
            top: 0,
            left: "50%",
            backgroundColor: UItheme.palette.primary.value,
            position: "absolute",
            transform:
              "translateX(-50%) rotate(0) skewX(0) skewY(0) scaleX(1) scaleY(1)",
          }}
        />
      </div>
    </div>
  );
}
