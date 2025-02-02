use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905159: FileFormat = FileFormat {
    id: 29_905_159,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System transport file",
    extensions: &["stx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
