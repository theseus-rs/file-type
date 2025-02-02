use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3570443: FileFormat = FileFormat {
    id: 3_570_443,
    source_type: SourceType::Wikidata,
    name: "Xtremsplit file format",
    extensions: &["xtm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
