use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114093986: FileFormat = FileFormat {
    id: 114_093_986,
    puid: "wikidata/114093986",
    name: "Sony SML File",
    extensions: &["sml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
