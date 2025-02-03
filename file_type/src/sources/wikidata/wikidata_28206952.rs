use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206952: FileFormat = FileFormat {
    id: 28_206_952,
    source_type: SourceType::Wikidata,
    name: "PhotoDeluxe",
    extensions: &["pdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
