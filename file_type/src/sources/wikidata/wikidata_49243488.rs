use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49243488: FileFormat = FileFormat {
    id: 49_243_488,
    source_type: SourceType::Wikidata,
    name: "License file",
    extensions: &["lic"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
