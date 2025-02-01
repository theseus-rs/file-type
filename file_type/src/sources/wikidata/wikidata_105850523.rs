use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850523: FileFormat = FileFormat {
    id: 105_850_523,
    puid: "wikidata/105850523",
    name: "Camtasia Studio Project",
    extensions: &["camproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
