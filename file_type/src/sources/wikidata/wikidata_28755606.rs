use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28755606: FileFormat = FileFormat {
    id: 28_755_606,
    source_type: SourceType::Wikidata,
    name: "Exact Yearbook DAT file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
