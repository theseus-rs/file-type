use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7311459: FileFormat = FileFormat {
    id: 7_311_459,
    puid: "wikidata/7311459",
    name: "Relocatable Object Module Format",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
