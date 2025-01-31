use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66141873: FileFormat = FileFormat {
    id: 66_141_873,
    puid: "wikidata/66141873",
    name: "MDE file format",
    extensions: &["mde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
