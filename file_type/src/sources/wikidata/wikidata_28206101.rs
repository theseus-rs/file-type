use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206101: FileFormat = FileFormat {
    id: 28_206_101,
    puid: "wikidata/28206101",
    name: "FaceSaver",
    extensions: &["fac", "face"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
