use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125297151: FileFormat = FileFormat {
    id: 125_297_151,
    puid: "wikidata/125297151",
    name: "cdb format",
    extensions: &["cdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
