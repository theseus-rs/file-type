use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122904069: FileFormat = FileFormat {
    id: 122_904_069,
    puid: "wikidata/122904069",
    name: "MBTiles",
    extensions: &["mbtiles"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
