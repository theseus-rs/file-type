use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28532082: FileFormat = FileFormat {
    id: 28_532_082,
    puid: "wikidata/28532082",
    name: "CAChe MolStruct",
    extensions: &["cac", "cache"],
    media_types: &["chemical/x-cache", "chemical/x-cache"],
    internal_signatures: &[],
    related_formats: &[],
};
