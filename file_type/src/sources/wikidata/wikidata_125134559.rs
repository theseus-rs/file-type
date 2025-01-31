use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125134559: FileFormat = FileFormat {
    id: 125_134_559,
    puid: "wikidata/125134559",
    name: "YAM Dictionary",
    extensions: &["glossary"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
