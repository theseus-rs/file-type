use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105582538: FileFormat = FileFormat {
    id: 105_582_538,
    puid: "wikidata/105582538",
    name: "JavaScript modules",
    extensions: &["mjs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
