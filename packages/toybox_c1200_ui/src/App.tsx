import { useEffect, useState } from "react";
import {
  Grid,
  Module,
  BadgeLegacy,
  Select,
  Slider,
  Description,
  ToggleList,
} from "altea";
import { Header, HeaderCenter, HeaderLeft, HeaderRight } from "./header";
import View from "./view";
import Container from "./container";
import Toybox from "./icon/toybox";
import presets from "./presets";

declare global {
  interface Window {
    onPluginMessage: (msg: any) => void;
    onPluginMessageInternal: (msg: any) => void;
    sendToPlugin: (msg: any) => void;
    ipc: any;
  }
}

const sendToPlugin = (msg: any) => {
  window.ipc.postMessage(JSON.stringify(msg));
};

function App() {
  const [preset, setPreset] = useState<string>("");
  const [output, setOutput] = useState<number>(0.5);
  const [reverbType, setReverbType] = useState<string>("");
  const [filterType, setFilterType] = useState<string>("");

  useEffect(() => {
    window.sendToPlugin = sendToPlugin;
    window.onPluginMessageInternal = function (msg) {
      const json = JSON.parse(msg);
      window.onPluginMessage && window.onPluginMessage(json);
    };

    window.onPluginMessage = (msg: any) => {
      if (Array.isArray(msg)) {
        msg.forEach((item) => {
          handlePluginMessage(item);
        });
      } else {
        handlePluginMessage(msg);
      }
    };

    const handlePluginMessage = (msg: any) => {
      switch (msg.type) {
        case "preset_change": {
          setPreset(msg.value);
          break;
        }
        case "output_gain_changed": {
          setOutput(msg.value);
          break;
        }
        case "reverb_type_changed": {
          setReverbType(msg.value);
          break;
        }
        case "filter_type_changed": {
          setFilterType(msg.value);
          break;
        }
      }
    };

    sendToPlugin({ type: "Init" });
  }, []);

  return (
    <View>
      <Header>
        <HeaderLeft>
          <BadgeLegacy size="md" icon={<Toybox />}>
            Toybox C1200
          </BadgeLegacy>
        </HeaderLeft>
        <HeaderCenter>
          <Select
            type="success"
            value={preset}
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
        </HeaderCenter>
        <HeaderRight>
          <Slider
            hideValue
            scale={0.5}
            max={1.0}
            min={0}
            onChange={(value) => {
              sendToPlugin({ type: "SetOutputGain", value: value });
            }}
            value={output}
            step={0.01}
          />
        </HeaderRight>
      </Header>
      <Container>
        <Grid.Container
          height={"100%"}
          width={"100%"}
          justify="space-around"
          style={{ padding: "10px", gap: "10px" }}
        >
          <Grid xs={7}>
            <Module
              name="Filter"
              footer={
                <Description
                  title={
                    <span style={{ margin: "-2px 6px" }}>FILTER TYPE</span>
                  }
                  content={
                    <ToggleList
                      margin={0.2}
                      scale={0.1}
                      width={"98%"}
                      value={filterType}
                      onChange={(value) =>
                        sendToPlugin({
                          type: "SetFilterType",
                          preset: value,
                        })
                      }
                    >
                      <ToggleList.Item value="Off">Off</ToggleList.Item>
                      <ToggleList.Item value="LowPass">LP</ToggleList.Item>
                      <ToggleList.Item value="HighPass">HP</ToggleList.Item>
                      <ToggleList.Item value="BandPass">BP</ToggleList.Item>
                    </ToggleList>
                  }
                />
              }
            >
              hello
            </Module>
          </Grid>
          <Grid xs={4}>
            <Module name="Vibrato">hello</Module>
          </Grid>
          <Grid xs={8}>
            <Module name="Chorus">hello</Module>
          </Grid>
          <Grid xs={4}>
            <Module
              name="Reverb"
              footer={
                <Description
                  title={
                    <span style={{ margin: "-2px 6px" }}>REVERB TYPE</span>
                  }
                  content={
                    <ToggleList
                      marginLeft={"1px"}
                      width={"126px"}
                      margin={0.2}
                      scale={0.1}
                      value={reverbType}
                      onChange={(value) =>
                        sendToPlugin({
                          type: "SetReverbType",
                          preset: value,
                        })
                      }
                    >
                      <ToggleList.Item value="Freeverb">Free</ToggleList.Item>
                      <ToggleList.Item value="Moorer">Mrrf</ToggleList.Item>
                    </ToggleList>
                  }
                />
              }
            >
              hello
            </Module>
          </Grid>
        </Grid.Container>
      </Container>
    </View>
  );
}

export default App;
