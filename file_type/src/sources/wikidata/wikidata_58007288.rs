use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58007288: FileFormat = FileFormat {
    id: 58_007_288,
    puid: "wikidata/58007288",
    name: "VBScript file",
    extensions: &["vbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
