use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28777715: FileFormat = FileFormat {
    id: 28_777_715,
    source_type: SourceType::Wikidata,
    name: "NSIS",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
