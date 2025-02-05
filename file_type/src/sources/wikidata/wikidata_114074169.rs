use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114074169: FileFormat = FileFormat {
    id: 114_074_169,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Text, version 1.3",
    extensions: &["odt"],
    media_types: &["application/vnd.oasis.opendocument.text"],
    signatures: &[],
    related_formats: &[],
};
