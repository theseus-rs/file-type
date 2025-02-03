use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119818987: FileFormat = FileFormat {
    id: 119_818_987,
    source_type: SourceType::Wikidata,
    name: "Final Draft AV Document",
    extensions: &["xav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
