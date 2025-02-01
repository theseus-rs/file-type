use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85513647: FileFormat = FileFormat {
    id: 85_513_647,
    puid: "wikidata/85513647",
    name: "Cindex Document, version 4",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
