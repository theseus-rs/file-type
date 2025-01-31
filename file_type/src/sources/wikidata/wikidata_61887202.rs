use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61887202: FileFormat = FileFormat {
    id: 61_887_202,
    puid: "wikidata/61887202",
    name: "EndNote Style File",
    extensions: &["ens"],
    media_types: &["application/x-endnote-style"],
    internal_signatures: &[],
    related_formats: &[],
};
