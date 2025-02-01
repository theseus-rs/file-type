use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_82521957: FileFormat = FileFormat {
    id: 82_521_957,
    puid: "wikidata/82521957",
    name: "Portable Voice format",
    extensions: &["pvf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
