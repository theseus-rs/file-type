use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51954279: FileFormat = FileFormat {
    id: 51_954_279,
    source_type: SourceType::Wikidata,
    name: "Autodesk Animator CEL File Format",
    extensions: &["cel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
