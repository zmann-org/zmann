import { useState } from "react";
import { Grid, Module, BadgeLegacy, Select, Slider, Button } from "altea";
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

function App() {
  const [preset, setPreset] = useState<string>("ElecOrgan4");
  // const [output, setOutput] = useState<number>(0.5);
  return (
    <View>
      <Header>
        <HeaderLeft>
          <BadgeLegacy size="md" icon={<Toybox />}>
            Toybox C1200
          </BadgeLegacy>
        </HeaderLeft>
        <HeaderCenter>
          <Select type="success" value={preset}>
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
            value={0.5}
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
		  <Module name="Filter"><Button onClick={() => setPreset('FrenchHorn1')}></Button></Module>
          </Grid>
          <Grid xs={4}>
            <Module name="Vibrato">hello</Module>
          </Grid>
          <Grid xs={8}>
            <Module name="Chorus">hello</Module>
          </Grid>
          <Grid xs={4}>
            <Module name="Reverb">hello</Module>
          </Grid>
        </Grid.Container>
      </Container>
    </View>
  );
}

export default App;
