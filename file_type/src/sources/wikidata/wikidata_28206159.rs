use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206159: FileFormat = FileFormat {
    id: 28_206_159,
    source_type: SourceType::Wikidata,
    name: "G9B",
    extensions: &["g9b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
