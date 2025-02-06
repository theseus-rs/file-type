use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115241368: FileFormat = FileFormat {
    id: 115_241_368,
    source_type: SourceType::Wikidata,
    name: "3D Builder Project",
    extensions: &["b3d"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
