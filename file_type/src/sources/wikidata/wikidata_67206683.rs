use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67206683: FileFormat = FileFormat {
    id: 67_206_683,
    source_type: SourceType::Wikidata,
    name: "VRML Worlds",
    extensions: &["3dv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
