use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34745761: FileFormat = FileFormat {
    id: 34_745_761,
    puid: "wikidata/34745761",
    name: "StarCraft group file",
    extensions: &["grp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
