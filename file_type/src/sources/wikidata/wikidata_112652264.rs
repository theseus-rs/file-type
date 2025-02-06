use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652264: FileFormat = FileFormat {
    id: 112_652_264,
    source_type: SourceType::Wikidata,
    name: "Autodesk SketchBook Animation File",
    extensions: &["skba"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
