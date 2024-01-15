"use client";
import { useEffect, useState } from "react";
import React, { ReactNode } from "react";
import {
  Grid,
  Select,
  Slider,
  Tag,
  Card,
  useTheme,
  useScale,
  ToggleList,
} from "@himalaya-ui/core";
import {
  Grid as GridIcon,
  HelpCircle,
  Settings,
} from "@himalaya-ui/core/icons";
import { KnobDecorative } from "@/lib/Knob";
import { KnobBase } from "@/lib/percentage/KnobBase";

declare global {
  interface Window {
    onPluginMessage: (msg: any) => void;
    onPluginMessageInternal: (msg: any) => void;
    sendToPlugin: (msg: any) => void;
    ipc: any;
  }
}

export default function Home() {
  const theme = useTheme();
  const presets = [
    { value: "Accordion", name: "Accordion" },
    { value: "AltoSax", name: "Alto Sax" },
    { value: "Bandoneon", name: "Bandoneon" },
    { value: "Brass1", name: "Brass 1" },
    { value: "Brass2", name: "Brass 2" },
    { value: "BrassEnsemble", name: "Brass Ensemble" },
    { value: "Cello", name: "Cello" },
    { value: "ChurchOrgan", name: "Church Organ" },
    { value: "Clarinet", name: "Clarinet" },
    { value: "ElecOrgan1", name: "Electric Organ 1" },
    { value: "ElecOrgan2", name: "Electric Organ 2" },
    { value: "ElecOrgan3", name: "Electric Organ 3" },
    { value: "ElecOrgan4", name: "Electric Organ 4" },
    { value: "Flute", name: "Flute" },
    { value: "FrenchHorn1", name: "French Horn 1" },
    { value: "FrenchHorn2", name: "French Horn 2" },
    { value: "Harmonica", name: "Harmonica" },
    { value: "Harp", name: "Harp" },
    { value: "Oboe", name: "Oboe" },
    { value: "Piccolo", name: "Piccolo" },
    { value: "PipeOrgan", name: "Pipe Organ" },
    { value: "Recorder", name: "Recorder" },
    { value: "ReedOrgan", name: "Reed Organ" },
    { value: "SopranoSax", name: "SopranoSax" },
    { value: "Soundtrack", name: "Soundtrack" },
    { value: "Strings1", name: "Strings 1" },
    { value: "Strings2", name: "Strings 2" },
    { value: "Strings3", name: "Strings 3" },
    { value: "SynPad1", name: "Synth Pad 1" },
    { value: "SynPad2", name: "Synth Pad 2" },
    { value: "SynPad3", name: "Synth Pad 3" },
    { value: "TenorSax", name: "Tenor Sax" },
    { value: "Trumpet", name: "Trumpet" },
    { value: "Tuba", name: "Tuba" },
    { value: "Violin", name: "Violin" },
  ];

  const sendToPlugin = (msg: any) => {
    window.ipc.postMessage(JSON.stringify(msg));
  };

  const [reverbDryWetValue, setreverbDryWetValue] = useState<number>(50);
  const [presetValue, setpresetValue] = useState<string>("");

  useEffect(() => {
    window.sendToPlugin = sendToPlugin;
    window.onPluginMessageInternal = function (msg) {
      const json = JSON.parse(msg);
      window.onPluginMessage && window.onPluginMessage(json);
    };
    window.onPluginMessage = (msg: any) => {
      switch (msg.type) {
        case "preset_change": {
          setpresetValue(msg.value);
          break;
        }
        case "reverb_dry_wet_change": {
          console.log(msg.value * 100);
          setreverbDryWetValue(msg.value * 100);
          break;
        }
      }
    };
    sendToPlugin({ type: "Init" });
  }, []);
  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      <nav
        style={{
          backgroundColor: theme.palette.accents_0,
          borderBottom: "1px solid " + theme.palette.secondary.value,
          height: "48px",
          display: "grid",
          gridTemplateColumns: "auto 1fr auto",
          alignItems: "center",
          zIndex: 100,
        }}
      >
        <div
          style={{
            justifySelf: "start",
            marginLeft: 12,
            display: "flex",
            verticalAlign: "middle",
            gap: "5px",
          }}
        >
          ZMANN
          <Tag type="primary" scale={0.6}>
            Alpha
          </Tag>
        </div>
        <div style={{ justifySelf: "center", minWidth: "200px" }}>
          <Select
            style={{ width: "100%" }}
            icon={(props) => <GridIcon size={18} {...props} />}
            type="primary"
            value={presetValue}
            onChange={(value) =>
              sendToPlugin({
                type: "SetPreset",
                preset: value,
              })
            }
          >
            {presets.map((preset) => (
              <Select.Option key={preset.value} value={preset.value}>
                {preset.name}
              </Select.Option>
            ))}
          </Select>
        </div>
        <div
          style={{
            justifySelf: "end",
            display: "flex",
            alignItems: "center",
            gap: "20px",
            marginRight: "20px",
          }}
        >
          <div style={{ fontSize: 12 }}>Volume</div>
          <Slider initialValue={1} scale={0.5} width={"100px"} step={10} />
          <HelpCircle />
          <Settings />
        </div>
      </nav>
      <main
        style={{
          height: "calc(100% - 48px)",
          width: "100%",
          paddingBottom: "0",
        }}
      >
        <Grid.Container
          height={"100%"}
          width={"100%"}
          justify="space-around"
          style={{ padding: "10px", gap: "10px" }}
        >
          <Grid xs={4}>
            <Module name="Filter" footer={<ToggleList value="test">
              <ToggleList.Item value="test">List</ToggleList.Item>
              <ToggleList.Item value="test2">Grid</ToggleList.Item>
            </ToggleList>}>
              <div style={{ height: '100%', display: 'flex', flexDirection: 'column', justifyContent: 'space-between' }}>
                <div>content</div>
              </div>
            </Module>
          </Grid>
          <Grid xs={4}>
            <Module name="Vibrato"></Module>
          </Grid>
          <Grid xs={8}>
            <Module name="Chorus"></Module>
          </Grid>
          <Grid xs={7}>
            <Module name="Reverb"><KnobBase onValueRawChange={(value) => {
              sendToPlugin({
                type: "SetReverbDryWet",
                value: (value / 100),
              })
            }} label='Dry/Wet' value01={reverbDryWetValue / 100} /></Module>
          </Grid>
        </Grid.Container>
      </main>
    </div>
  );
}

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
            position: 'absolute',
            display: 'flex',
            justifyContent: 'center',
            bottom: 0,
            marginTop: 'auto', // Add this line
          }}
        >
          {footer}
        </div>
      )}
    </Card>
  );
};
