use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84942649: FileFormat = FileFormat {
    id: 84_942_649,
    puid: "wikidata/84942649",
    name: "Sony PictureGear Studio PhotoAlbum",
    extensions: &["amd", "amu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
