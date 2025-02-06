use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27203601: FileFormat = FileFormat {
    id: 27_203_601,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Text, version 1.2",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    signatures: &[],
    related_formats: &[],
};
