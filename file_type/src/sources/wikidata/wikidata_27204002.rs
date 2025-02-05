use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27204002: FileFormat = FileFormat {
    id: 27_204_002,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Presentation, version 1.2",
    extensions: &["odp"],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    signatures: &[],
    related_formats: &[],
};
