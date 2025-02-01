use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124970064: FileFormat = FileFormat {
    id: 124_970_064,
    puid: "wikidata/124970064",
    name: "MIX index file",
    extensions: &["mixindex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
