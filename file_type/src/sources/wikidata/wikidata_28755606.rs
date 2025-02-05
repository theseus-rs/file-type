use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28755606: FileFormat = FileFormat {
    id: 28_755_606,
    source_type: SourceType::Wikidata,
    name: "Exact Yearbook DAT file",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
