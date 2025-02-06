use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28755621: FileFormat = FileFormat {
    id: 28_755_621,
    source_type: SourceType::Wikidata,
    name: "Exact Yearbook ID file",
    extensions: &["id"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
