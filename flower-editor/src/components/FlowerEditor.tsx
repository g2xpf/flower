import { useResizeDetector } from "react-resize-detector";
import { useFlowerEditor } from "../lib/flowerEditor";
import { Grid, Typography, Button } from "@mui/material";
import { styled } from "@mui/material/styles";

const PathTypography = styled("div")({
  overflow: "scroll",
  whiteSpace: "nowrap",
  width: 400,
});

export default function FlowerEditor() {
  const { width, height, ref: canvasParentRef } = useResizeDetector();

  const { ref, saveFlow, loadFlow, handleKeyInput, createFlow, flowPath } =
    useFlowerEditor({
      width,
      height,
    });

  const instructions = [
    {
      name: "Load",
      callback: loadFlow,
    },
    {
      name: "Save",
      callback: saveFlow,
    },
    {
      name: "Create",
      callback: createFlow,
    },
  ];

  return (
    <Grid container direction="column" sx={{ width: "100%", height: "100%" }}>
      <Grid
        item
        xs={1}
        container
        direction="row"
        justifyContent="space-evenly"
        sx={{ maxWidth: "100% !important", maxHeight: `${100 / 12}%` }}
      >
        <Grid item xs={6} container justifyContent="center" alignItems="center">
          <Grid item>
            <PathTypography>
              <Typography variant="h6">{`path: ${flowPath}`}</Typography>
            </PathTypography>
          </Grid>
        </Grid>
        {instructions.map((instruction, index) => (
          <Grid
            key={index}
            item
            xs={(12 - 6) / instructions.length}
            container
            justifyContent="center"
            alignItems="center"
          >
            <Grid item>
              <Button variant="contained" onClick={instruction.callback}>
                {instruction.name}
              </Button>
            </Grid>
          </Grid>
        ))}
      </Grid>
      <Grid
        item
        xs
        sx={{ maxWidth: "100% !important", maxHeight: `${(100 / 12) * 11}%` }}
      >
        <div ref={canvasParentRef} style={{ width: "100%", height: "100%" }}>
          {!!width && !!height && (
            <canvas ref={ref} onKeyDown={handleKeyInput} />
          )}
        </div>
      </Grid>
    </Grid>
  );
}
