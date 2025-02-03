use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27203907: FileFormat = FileFormat {
    id: 27_203_907,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Presentation, version 1.0",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
