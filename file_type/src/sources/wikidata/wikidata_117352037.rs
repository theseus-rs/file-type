use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117352037: FileFormat = FileFormat {
    id: 117_352_037,
    puid: "wikidata/117352037",
    name: "OrCAD project",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
