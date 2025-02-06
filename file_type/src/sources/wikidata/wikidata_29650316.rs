use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650316: FileFormat = FileFormat {
    id: 29_650_316,
    source_type: SourceType::Wikidata,
    name: "Packed Font File Format",
    extensions: &["pk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
