use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117259797: FileFormat = FileFormat {
    id: 117_259_797,
    puid: "wikidata/117259797",
    name: "TurboCAD 3D Model file",
    extensions: &["mdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
