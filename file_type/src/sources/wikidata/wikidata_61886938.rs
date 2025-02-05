use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61886938: FileFormat = FileFormat {
    id: 61_886_938,
    source_type: SourceType::Wikidata,
    name: "Portable Form File",
    extensions: &["pff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
