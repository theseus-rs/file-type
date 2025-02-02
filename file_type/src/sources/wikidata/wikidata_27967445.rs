use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967445: FileFormat = FileFormat {
    id: 27_967_445,
    source_type: SourceType::Wikidata,
    name: "Autodesk Animator Pro FLIC",
    extensions: &["flc"],
    media_types: &["video/flc"],
    internal_signatures: &[],
    related_formats: &[],
};
