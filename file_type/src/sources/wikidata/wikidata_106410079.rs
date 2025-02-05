use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_106410079: FileFormat = FileFormat {
    id: 106_410_079,
    source_type: SourceType::Wikidata,
    name: "MIRAX/MRXS",
    extensions: &["mrxs"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
