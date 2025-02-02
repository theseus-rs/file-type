use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114059857: FileFormat = FileFormat {
    id: 114_059_857,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Presentation, version 1.3",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
