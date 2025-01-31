use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650309: FileFormat = FileFormat {
    id: 29_650_309,
    puid: "wikidata/29650309",
    name: "PQA",
    extensions: &["pqa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
