use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61963251: FileFormat = FileFormat {
    id: 61_963_251,
    puid: "wikidata/61963251",
    name: "Internet Data Query File",
    extensions: &["idq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
