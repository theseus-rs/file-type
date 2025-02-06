use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3563777: FileFormat = FileFormat {
    id: 3_563_777,
    source_type: SourceType::Wikidata,
    name: "MicroDVD",
    extensions: &["sub"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
