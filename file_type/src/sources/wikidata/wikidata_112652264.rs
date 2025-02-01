use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652264: FileFormat = FileFormat {
    id: 112_652_264,
    puid: "wikidata/112652264",
    name: "Autodesk SketchBook Animation File",
    extensions: &["skba"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
