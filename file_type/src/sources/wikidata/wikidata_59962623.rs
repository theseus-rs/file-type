use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59962623: FileFormat = FileFormat {
    id: 59_962_623,
    source_type: SourceType::Wikidata,
    name: "Autodesk Animator (FlicLib)",
    extensions: &["fli"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
