use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27355592: FileFormat = FileFormat {
    id: 27_355_592,
    puid: "wikidata/27355592",
    name: "ADRG Geo Data File",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
