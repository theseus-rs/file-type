use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27960118: FileFormat = FileFormat {
    id: 27_960_118,
    source_type: SourceType::Wikidata,
    name: "Sony Wave64",
    extensions: &["w64"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
