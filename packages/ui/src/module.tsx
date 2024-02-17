import { ReactNode } from "react";
import { Card, useTheme, useScale } from "@himalaya-ui/core";
interface ModuleProps {
  children?: ReactNode;
  footer?: ReactNode;
  name?: string;
}

const Module: React.FC<ModuleProps> = ({ children, footer, name }) => {
  const theme = useTheme();
  const { SCALES } = useScale();
  return (
    <Card
      style={{
        height: "100%",
        width: "100%",
        background: theme.palette.accents_0 + "B2",
        display: "flex",
        flexDirection: "column",
      }}
    >
      {name && (
        <header>
          <div
            style={{
              border: `1px solid ${theme.palette.border}`,
              backgroundColor: theme.palette.accents_0,
              color: theme.palette.accents_5,
              height: "auto",
              lineHeight: "1.35em",
              display: "inline-flex",
              alignItems: "center",
              fontSize: SCALES.font(0.8125),
              padding: `${SCALES.font(0.32)} ${SCALES.font(0.5)} ${SCALES.font(
                0.32
              )} ${SCALES.font(0.5)}`,
              width: "auto",
              borderTopLeftRadius: `calc(${theme.style.radius} - 1px)`,
              borderBottomRightRadius: theme.style.radius,
              textTransform: "uppercase",
              marginTop: "-1px",
              marginLeft: "-1px",
            }}
          >
            {name}
          </div>
        </header>
      )}
      <Card.Content>{children}</Card.Content>
      {footer && (
        <div
          style={{
            display: "flex",
            justifyContent: "center",
            marginTop: "auto",
          }}
        >
          {footer}
        </div>
      )}
    </Card>
  );
};

export default Module;
