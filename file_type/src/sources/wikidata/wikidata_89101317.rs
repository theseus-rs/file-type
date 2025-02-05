use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_89101317: FileFormat = FileFormat {
    id: 89_101_317,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Gold Project 1",
    extensions: &["ban", "cal", "car", "let", "sig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
