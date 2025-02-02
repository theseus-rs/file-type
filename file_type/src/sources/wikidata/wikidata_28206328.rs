use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206328: FileFormat = FileFormat {
    id: 28_206_328,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Red Component",
    extensions: &["r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
