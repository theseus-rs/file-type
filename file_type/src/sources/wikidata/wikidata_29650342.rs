use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650342: FileFormat = FileFormat {
    id: 29_650_342,
    source_type: SourceType::Wikidata,
    name: "PEM encoded certificate",
    extensions: &["cer", "crt", "pem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
