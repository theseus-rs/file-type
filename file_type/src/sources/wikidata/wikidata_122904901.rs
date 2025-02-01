use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122904901: FileFormat = FileFormat {
    id: 122_904_901,
    puid: "wikidata/122904901",
    name: "PMTiles",
    extensions: &["pmtiles"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
