use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34307182: FileFormat = FileFormat {
    id: 34_307_182,
    source_type: SourceType::Wikidata,
    name: "ScreenWriter II",
    extensions: &["text"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
