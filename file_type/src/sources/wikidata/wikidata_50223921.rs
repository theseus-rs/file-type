use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50223921: FileFormat = FileFormat {
    id: 50_223_921,
    source_type: SourceType::Wikidata,
    name: "Adobe Air",
    extensions: &["air"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
