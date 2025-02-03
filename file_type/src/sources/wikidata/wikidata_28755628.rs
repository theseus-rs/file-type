use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28755628: FileFormat = FileFormat {
    id: 28_755_628,
    source_type: SourceType::Wikidata,
    name: "Exact Yearbook LST file",
    extensions: &["lst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
