use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851526: FileFormat = FileFormat {
    id: 105_851_526,
    puid: "wikidata/105851526",
    name: "Camtasia Project",
    extensions: &["tscproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
