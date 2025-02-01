use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852994: FileFormat = FileFormat {
    id: 105_852_994,
    puid: "wikidata/105852994",
    name: "SatcoDX channel list",
    extensions: &["sdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
