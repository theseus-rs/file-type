use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73160161: FileFormat = FileFormat {
    id: 73_160_161,
    source_type: SourceType::Wikidata,
    name: "Visio Drawing Template",
    extensions: &["vst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
