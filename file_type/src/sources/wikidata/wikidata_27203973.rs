use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27203973: FileFormat = FileFormat {
    id: 27_203_973,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Presentation, version 1.1",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    signatures: &[],
    related_formats: &[],
};
