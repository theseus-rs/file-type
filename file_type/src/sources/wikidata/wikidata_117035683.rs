use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117035683: FileFormat = FileFormat {
    id: 117_035_683,
    puid: "wikidata/117035683",
    name: "FloorPlan file",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
