use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125255794: FileFormat = FileFormat {
    id: 125_255_794,
    source_type: SourceType::Wikidata,
    name: "CombiTimeTable",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
