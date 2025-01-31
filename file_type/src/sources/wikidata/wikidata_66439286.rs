use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66439286: FileFormat = FileFormat {
    id: 66_439_286,
    puid: "wikidata/66439286",
    name: "DisplayWrite Document file format, version 5",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
