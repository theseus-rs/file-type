use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206513: FileFormat = FileFormat {
    id: 28_206_513,
    source_type: SourceType::Wikidata,
    name: "LSS16",
    extensions: &["16", "lss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
