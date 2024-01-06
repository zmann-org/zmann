"use client";

import { Card, ContentLayout, Grid } from "@himalaya-ui/core";

export default function Home() {
  return (
    <ContentLayout style={{height: '92%'}}>
      <Grid.Container gap={2} justify="space-between" height="100%">
        <Grid xs={6}>
          <Card shadow width="100%" />
        </Grid>
        <Grid xs={6}>
          <Card shadow width="100%" />
        </Grid>
        <Grid xs={6}>
          <Card shadow width="100%" />
        </Grid>
        <Grid xs={6}>
          <Card shadow width="100%" />
        </Grid>
      </Grid.Container>
    </ContentLayout>
  );
}
