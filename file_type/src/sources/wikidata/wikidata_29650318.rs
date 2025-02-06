use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650318: FileFormat = FileFormat {
    id: 29_650_318,
    source_type: SourceType::Wikidata,
    name: "PKPass",
    extensions: &["pkpass"],
    media_types: &["application/vnd.apple.pkpass"],
    signatures: &[],
    related_formats: &[],
};
