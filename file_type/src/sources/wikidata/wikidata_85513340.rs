use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85513340: FileFormat = FileFormat {
    id: 85_513_340,
    puid: "wikidata/85513340",
    name: "Cindex Document, version 3",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
