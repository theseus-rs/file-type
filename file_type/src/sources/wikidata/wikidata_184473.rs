use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_184473: FileFormat = FileFormat {
    id: 184_473,
    source_type: SourceType::Wikidata,
    name: "OpenDocument",
    extensions: &["fodt", "odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    signatures: &[],
    related_formats: &[],
};
