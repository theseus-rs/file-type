use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119857023: FileFormat = FileFormat {
    id: 119_857_023,
    source_type: SourceType::Wikidata,
    name: "Map Template",
    extensions: &["stt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
