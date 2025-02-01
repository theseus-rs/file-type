use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_382011: FileFormat = FileFormat {
    id: 382_011,
    puid: "wikidata/382011",
    name: "Program information file",
    extensions: &["pif"],
    media_types: &["application/x-pif"],
    internal_signatures: &[],
    related_formats: &[],
};
