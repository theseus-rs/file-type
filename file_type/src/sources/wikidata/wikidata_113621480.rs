use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113621480: FileFormat = FileFormat {
    id: 113_621_480,
    source_type: SourceType::Wikidata,
    name: "LoadRunner Analysis file",
    extensions: &["lra"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
