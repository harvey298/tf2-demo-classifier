# TF2 Demo Classifier

Classifier for TF2 demos!

## How to Use

1. Paste the path to a demo file.
2. Type 'y' if the user is a cheater.
3. Type 's' to stop reading the demo.

## Save Format - Aitl

A `.aitil` file contains all the training labels and training data. It basically consists of a demo file with a JSON file at the start.

The file format is composed of three parts:

1. The first part ends with a null byte (`\0`), which represents the header size.
2. The second part starts from the end of the first part (the null byte) up to the header size. This is the header, which contains the training labels (see to `src/atil.rs`).
3. The third part is the demo file itself.

The save format used is compatible with [Seer-tf2](https://github.com/harvey298/seer-tf2).
