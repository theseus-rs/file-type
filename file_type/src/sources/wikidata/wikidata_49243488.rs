use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_49243488: FileFormat = FileFormat {
    id: 49_243_488,
    source_type: SourceType::Wikidata,
    name: "License file",
    extensions: &["lic"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
