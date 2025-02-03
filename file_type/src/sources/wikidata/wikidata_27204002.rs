use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27204002: FileFormat = FileFormat {
    id: 27_204_002,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Presentation, version 1.2",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
