use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119975385: FileFormat = FileFormat {
    id: 119_975_385,
    source_type: SourceType::Wikidata,
    name: "Style Template",
    extensions: &["tps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
