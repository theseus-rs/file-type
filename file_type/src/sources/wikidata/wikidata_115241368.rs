use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115241368: FileFormat = FileFormat {
    id: 115_241_368,
    source_type: SourceType::Wikidata,
    name: "3D Builder Project",
    extensions: &["b3d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
