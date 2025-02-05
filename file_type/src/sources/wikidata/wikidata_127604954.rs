use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127604954: FileFormat = FileFormat {
    id: 127_604_954,
    source_type: SourceType::Wikidata,
    name: "Arduino Sketch file",
    extensions: &["ino"],
    media_types: &["text/x-arduino"],
    signatures: &[],
    related_formats: &[],
};
