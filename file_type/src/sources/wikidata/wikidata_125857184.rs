use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125857184: FileFormat = FileFormat {
    id: 125_857_184,
    puid: "wikidata/125857184",
    name: "C-- source code file",
    extensions: &["c--"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
