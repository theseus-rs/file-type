use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59962623: FileFormat = FileFormat {
    id: 59_962_623,
    source_type: SourceType::Wikidata,
    name: "Autodesk Animator (FlicLib)",
    extensions: &["fli"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
