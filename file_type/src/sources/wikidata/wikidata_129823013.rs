use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129823013: FileFormat = FileFormat {
    id: 129_823_013,
    puid: "wikidata/129823013",
    name: "Inform 7 source code file",
    extensions: &["i7x", "ni"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
