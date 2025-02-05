use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650342: FileFormat = FileFormat {
    id: 29_650_342,
    source_type: SourceType::Wikidata,
    name: "PEM encoded certificate",
    extensions: &["cer", "crt", "pem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
