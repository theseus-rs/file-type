use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29944072: FileFormat = FileFormat {
    id: 29_944_072,
    source_type: SourceType::Wikidata,
    name: "Simple Voxels",
    extensions: &["svx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
