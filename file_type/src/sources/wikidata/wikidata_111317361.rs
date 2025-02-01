use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317361: FileFormat = FileFormat {
    id: 111_317_361,
    puid: "wikidata/111317361",
    name: "MAUD sample format",
    extensions: &["maud"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
