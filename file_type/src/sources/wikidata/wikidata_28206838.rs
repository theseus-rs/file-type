use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206838: FileFormat = FileFormat {
    id: 28_206_838,
    source_type: SourceType::Wikidata,
    name: "Palm bitmap",
    extensions: &["palm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
