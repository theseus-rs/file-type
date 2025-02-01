use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84087750: FileFormat = FileFormat {
    id: 84_087_750,
    puid: "wikidata/84087750",
    name: "FamilyTree Maker Database 1-4",
    extensions: &["fbk", "ftw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
