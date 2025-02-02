use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113365166: FileFormat = FileFormat {
    id: 113_365_166,
    source_type: SourceType::Wikidata,
    name: "Camtasia Recording",
    extensions: &["trec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
