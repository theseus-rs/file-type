use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73160161: FileFormat = FileFormat {
    id: 73_160_161,
    source_type: SourceType::Wikidata,
    name: "Visio Drawing Template",
    extensions: &["vst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
