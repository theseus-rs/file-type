use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206493: FileFormat = FileFormat {
    id: 28_206_493,
    source_type: SourceType::Wikidata,
    name: "Lightning Strike",
    extensions: &["cod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
