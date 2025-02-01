use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3063041: FileFormat = FileFormat {
    id: 3_063_041,
    puid: "wikidata/3063041",
    name: "Filmbox",
    extensions: &["fbx", "fbx"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
