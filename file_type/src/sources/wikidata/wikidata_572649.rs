use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_572649: FileFormat = FileFormat {
    id: 572_649,
    puid: "wikidata/572649",
    name: "Intel HEX",
    extensions: &["hex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
