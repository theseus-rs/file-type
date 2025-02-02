use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119857023: FileFormat = FileFormat {
    id: 119_857_023,
    source_type: SourceType::Wikidata,
    name: "Map Template",
    extensions: &["stt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
