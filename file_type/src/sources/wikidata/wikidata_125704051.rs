use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125704051: FileFormat = FileFormat {
    id: 125_704_051,
    puid: "wikidata/125704051",
    name: "StarDraw 2.0 file",
    extensions: &["sgv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
