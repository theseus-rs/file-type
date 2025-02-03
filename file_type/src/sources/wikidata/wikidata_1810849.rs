use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1810849: FileFormat = FileFormat {
    id: 1_810_849,
    source_type: SourceType::Wikidata,
    name: "XLIFF",
    extensions: &["xlf"],
    media_types: &["application/xliff+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
