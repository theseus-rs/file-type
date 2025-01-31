use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51801391: FileFormat = FileFormat {
    id: 51_801_391,
    puid: "wikidata/51801391",
    name: "AutoCAD Xref Log",
    extensions: &["xlg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
