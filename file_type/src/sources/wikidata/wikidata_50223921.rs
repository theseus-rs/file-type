use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50223921: FileFormat = FileFormat {
    id: 50_223_921,
    source_type: SourceType::Wikidata,
    name: "Adobe Air",
    extensions: &["air"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
