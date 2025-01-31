use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51837533: FileFormat = FileFormat {
    id: 51_837_533,
    puid: "wikidata/51837533",
    name: "Visual FoxPro Database Container File",
    extensions: &["dcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
