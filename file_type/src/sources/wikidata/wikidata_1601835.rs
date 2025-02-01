use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1601835: FileFormat = FileFormat {
    id: 1_601_835,
    puid: "wikidata/1601835",
    name: "Standard Test Data Format",
    extensions: &["std", "stdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
