use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858528: FileFormat = FileFormat {
    id: 105_858_528,
    source_type: SourceType::Wikidata,
    name: "PrintFox/Pagefox bitmap (640x800)",
    extensions: &["bin", "pg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
