use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117424649: FileFormat = FileFormat {
    id: 117_424_649,
    puid: "wikidata/117424649",
    name: "Stationery file",
    extensions: &["sta"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
