use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121788559: FileFormat = FileFormat {
    id: 121_788_559,
    puid: "wikidata/121788559",
    name: "Leapfrog Geo 3D Scene Format",
    extensions: &["lfsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
