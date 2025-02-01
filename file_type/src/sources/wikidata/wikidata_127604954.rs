use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127604954: FileFormat = FileFormat {
    id: 127_604_954,
    puid: "wikidata/127604954",
    name: "Arduino Sketch file",
    extensions: &["ino"],
    media_types: &["text/x-arduino"],
    internal_signatures: &[],
    related_formats: &[],
};
