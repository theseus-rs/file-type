use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84942649: FileFormat = FileFormat {
    id: 84_942_649,
    source_type: SourceType::Wikidata,
    name: "Sony PictureGear Studio PhotoAlbum",
    extensions: &["amd", "amu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
