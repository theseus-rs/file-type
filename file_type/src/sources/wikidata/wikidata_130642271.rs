use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130642271: FileFormat = FileFormat {
    id: 130_642_271,
    source_type: SourceType::Wikidata,
    name: "Roboconf graph file",
    extensions: &["graph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
