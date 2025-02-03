use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123483255: FileFormat = FileFormat {
    id: 123_483_255,
    source_type: SourceType::Wikidata,
    name: "C extension for CPython on Windows (.pyd)",
    extensions: &["pyd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
