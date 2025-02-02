use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3563777: FileFormat = FileFormat {
    id: 3_563_777,
    source_type: SourceType::Wikidata,
    name: "MicroDVD",
    extensions: &["sub"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
