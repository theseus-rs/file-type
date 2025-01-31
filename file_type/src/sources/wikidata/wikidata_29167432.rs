use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167432: FileFormat = FileFormat {
    id: 29_167_432,
    puid: "wikidata/29167432",
    name: "NuFX",
    extensions: &["bxy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
