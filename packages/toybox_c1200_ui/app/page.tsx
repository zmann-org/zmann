"use client";
import { useEffect, useRef, useState } from "react";
import React, { ReactNode } from "react";
import {
  Grid,
  Select,
  Slider,
  useTheme,
  ToggleList,
  Description,
  Button,
} from "@himalaya-ui/core";
import {
  Grid as GridIcon,
  HelpCircle,
  Settings,
} from "@himalaya-ui/core/icons";
import Badge from "@/lib/Badge";
import Toybox from "@/lib/toybox";
import Module from "@repo/ui/module";

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

  function degreeChange() {
    console.log(`event value is `);
  }

  const [reverbDryWetValue, setreverbDryWetValue] = useState<number>(0);
  const [reverbTypeValue, setreverbTypeValue] = useState<string>("");
  const [presetValue, setpresetValue] = useState<string>("");
  const inputRef = useRef(null);

  //@ts-ignore
  useEffect(() => {
    // Use the inputRef to interact with the third-party script
    // For example, if the third-party script is modifying the input, you might need to trigger a change event manually
    // inputRef.current.dispatchEvent(new Event('change'));

    // Or add event listeners directly to the inputRef
    const inputElement = inputRef.current;
    //@ts-ignore
    inputElement.addEventListener("input", handleInputChange);

    // Cleanup the event listener on component unmount
    return () => {
      //@ts-ignore
      inputElement.removeEventListener("input", handleInputChange);
    };
  }, []);

  //@ts-ignore
  const handleInputChange = (event) => {
    sendToPlugin({
      type: "SetReverbDryWet",
      value: Number(event.target.value),
    });
  };
  useEffect(() => {
    window.sendToPlugin = sendToPlugin;
    window.onPluginMessageInternal = function (msg) {
      const json = JSON.parse(msg);
      window.onPluginMessage && window.onPluginMessage(json);
    };
    window.onPluginMessage = (msg: any) => {
      console.log("[" + msg.type + "] {" + msg.value + "}");
      switch (msg.type) {
        case "preset_change": {
          setpresetValue(msg.value);
          break;
        }
        case "reverb_dry_wet_change": {
          setreverbDryWetValue(msg.value);
          break;
        }
        case "reverb_type_changed": {
          setreverbTypeValue(msg.value);
          break;
        }
      }
    };
    sendToPlugin({ type: "Init" });
  }, []);
  return (
    <>
      <div
        style={{ display: "flex", flexDirection: "column", height: "100vh" }}
      >
        <nav
          style={{
            backgroundColor: theme.palette.accents_0,
            borderBottom: "1px solid #515151",
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
            <Badge size="md" icon={<Toybox />}>
              Toybox C1200
            </Badge>
          </div>
          <div style={{ justifySelf: "center", minWidth: "200px" }}>
            <Select
              style={{ width: "100%" }}
              // icon={(props) => <GridIcon size={18} {...props} />}
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
              gap: "5px",
              marginRight: "10px",
            }}
          >
            <div style={{ fontSize: 12 }}>Volume</div>
            <Slider
              initialValue={1}
              scale={0.5}
              width={"100px"}
              max={30}
              min={-30}
              step={1}
            />
            <Button type="abort" auto icon={<HelpCircle />}></Button>
            <Button type="abort" auto icon={<Settings />}></Button>
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
            <Grid xs={7}>
              <Module name="Filter">filter</Module>
            </Grid>
            <Grid xs={4}>
              <Module name="Vibrato">hello</Module>
            </Grid>
            <Grid xs={8}>
              <Module name="Chorus"></Module>
            </Grid>
            <Grid xs={4}>
              <Module
                name="Reverb"
                footer={
                  <Description
                  style={{marginTop: '-24px'}}
                    title={
                      <span style={{ margin: "-2px 6px" }}>REVERB TYPE</span>
                    }
                    content={
                      <ToggleList
                        width={"95%"}
                        margin={0.2}
                        scale={0.1}
                        value={reverbTypeValue}
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
                <div
                  style={{
                    height: "100%",
                    display: "flex",
                    flexDirection: "column",
                    justifyContent: "space-between",
                    alignItems: "center", // Center child div horizontally
                  }}
                >
                  <div style={{ textAlign: "center", marginBottom: "5px" }}> {/* Center input-text */}
                    <input
                      type="range"
                      ref={inputRef}
                      value={reverbDryWetValue}
                      max={1}
                      min={0}
                      step={0.01}
                      className="input-knob"
                      data-src="https://i.imgur.com/K5NDNNK.png"
                      data-sprites="78"
                      onChange={(event) => {
                        console.log(event + "event");
                        sendToPlugin({
                          type: "SetReverbDryWet",
                          value: Number(event.target.value),
                        });
                      }}
                    />
                    <span className="input-text">DRY/WET</span>
                  </div>
                  <div style={{ textAlign: "center", marginBottom: "5px" }}> {/* Center input-text */}
                    <input
                      type="range"
                      max={1}
                      min={0}
                      step={0.01}
                      className="input-knob"
                      data-src="https://i.imgur.com/K5NDNNK.png"
                      data-sprites="78"
                      onChange={(event) => {
                        console.log(event + "event");
                      }}
                    />
                    <span className="input-text">ROOMSIZE</span>
                  </div>
                </div>
              </Module>
            </Grid>
          </Grid.Container>
        </main>
      </div>
      <style jsx>{`
        .input-text {
          font-size: 0.75em;
          line-height: 1em;
          margin-bottom: 0.5em;
          text-transform: uppercase;
          white-space: nowrap;
          color: #969698;
          font-weight: 500;
          display: -webkit-box;
          display: -webkit-flex;
          display: -ms-flexbox;
          display: flex;
        }
      `}</style>
    </>
  );
}