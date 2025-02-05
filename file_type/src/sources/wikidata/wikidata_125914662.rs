use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125914662: FileFormat = FileFormat {
    id: 125_914_662,
    source_type: SourceType::Wikidata,
    name: "Sandboxels Save File",
    extensions: &["sbxl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
