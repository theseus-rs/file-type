use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111667275: FileFormat = FileFormat {
    id: 111_667_275,
    puid: "wikidata/111667275",
    name: "OEW objectbase",
    extensions: &["oew"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
