use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111315908: FileFormat = FileFormat {
    id: 111_315_908,
    puid: "wikidata/111315908",
    name: "INRS-Telecommunications audio file",
    extensions: &["inrs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
