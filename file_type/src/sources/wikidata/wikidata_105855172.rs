use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855172: FileFormat = FileFormat {
    id: 105_855_172,
    puid: "wikidata/105855172",
    name: "File-Type Image",
    extensions: &["fti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
