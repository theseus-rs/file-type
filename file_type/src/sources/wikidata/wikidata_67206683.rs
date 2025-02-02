use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67206683: FileFormat = FileFormat {
    id: 67_206_683,
    source_type: SourceType::Wikidata,
    name: "VRML Worlds",
    extensions: &["3dv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
