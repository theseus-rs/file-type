use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919035: FileFormat = FileFormat {
    id: 28_919_035,
    source_type: SourceType::Wikidata,
    name: "Type-1 DV AVI",
    extensions: &["avi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
