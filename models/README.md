# Vision Models

This directory stores model cards and local development metadata for screenshot analysis.

Initial analyzer path:

1. User uploads a village screenshot.
2. OpenCV normalizes perspective, scale, and contrast.
3. ONNX Runtime detects town hall, defenses, walls, collectors, and storages.
4. Detections are mapped into an internal village grid.
5. AI analysis receives only derived grid facts and user-provided context.

Future YOLO models must include license, training data provenance, labels, confidence thresholds, and evaluation results.
