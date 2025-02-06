use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960087: FileFormat = FileFormat {
    id: 27_960_087,
    source_type: SourceType::Wikidata,
    name: "Memory Stick Voice",
    extensions: &["msv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
