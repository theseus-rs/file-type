use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113519455: FileFormat = FileFormat {
    id: 113_519_455,
    source_type: SourceType::Wikidata,
    name: "PageMaker Mac Document 6.0",
    extensions: &["pm6", "pt6"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
