use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7866417: FileFormat = FileFormat {
    id: 7_866_417,
    puid: "wikidata/7866417",
    name: "USGS DEM",
    extensions: &["dem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
