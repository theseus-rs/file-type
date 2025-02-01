use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66142150: FileFormat = FileFormat {
    id: 66_142_150,
    puid: "wikidata/66142150",
    name: "ADE file format",
    extensions: &["ade"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
