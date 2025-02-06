use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3570443: FileFormat = FileFormat {
    id: 3_570_443,
    source_type: SourceType::Wikidata,
    name: "Xtremsplit file format",
    extensions: &["xtm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
