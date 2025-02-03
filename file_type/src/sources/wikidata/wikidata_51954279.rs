use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51954279: FileFormat = FileFormat {
    id: 51_954_279,
    source_type: SourceType::Wikidata,
    name: "Autodesk Animator CEL File Format",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
