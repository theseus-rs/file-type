use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2145498: FileFormat = FileFormat {
    id: 2_145_498,
    puid: "wikidata/2145498",
    name: "Requirements Interchange Format",
    extensions: &["reqif", "reqifz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
