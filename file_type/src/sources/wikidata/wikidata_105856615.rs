use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856615: FileFormat = FileFormat {
    id: 105_856_615,
    puid: "wikidata/105856615",
    name: "WiX Project",
    extensions: &["wixproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
