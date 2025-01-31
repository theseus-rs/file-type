use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125692127: FileFormat = FileFormat {
    id: 125_692_127,
    puid: "wikidata/125692127",
    name: "OpenDocument Presentation Template",
    extensions: &["otp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
