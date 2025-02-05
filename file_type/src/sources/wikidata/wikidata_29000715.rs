use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000715: FileFormat = FileFormat {
    id: 29_000_715,
    source_type: SourceType::Wikidata,
    name: "Unreal model data file",
    extensions: &["3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
