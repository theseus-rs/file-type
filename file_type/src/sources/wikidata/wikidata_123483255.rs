use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123483255: FileFormat = FileFormat {
    id: 123_483_255,
    source_type: SourceType::Wikidata,
    name: "C extension for CPython on Windows (.pyd)",
    extensions: &["pyd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
