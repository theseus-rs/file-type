use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123483270: FileFormat = FileFormat {
    id: 123_483_270,
    source_type: SourceType::Wikidata,
    name: "executable Python zip archive (.pyz)",
    extensions: &["pyz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
