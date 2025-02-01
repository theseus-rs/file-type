use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284149: FileFormat = FileFormat {
    id: 111_284_149,
    puid: "wikidata/111284149",
    name: "Bells, Whistles, Sound Boards module",
    extensions: &["gdm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
