use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130367574: FileFormat = FileFormat {
    id: 130_367_574,
    puid: "wikidata/130367574",
    name: "Community Climate Model History Tape Format",
    extensions: &["ccm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
