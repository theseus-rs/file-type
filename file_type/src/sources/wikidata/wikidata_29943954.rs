use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29943954: FileFormat = FileFormat {
    id: 29_943_954,
    puid: "wikidata/29943954",
    name: "Statistical Package for the Social Sciences output file",
    extensions: &["spv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
