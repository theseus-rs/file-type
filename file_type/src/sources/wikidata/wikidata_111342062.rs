use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342062: FileFormat = FileFormat {
    id: 111_342_062,
    puid: "wikidata/111342062",
    name: "Melody Machine compressed SoundFont",
    extensions: &["sfark"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
