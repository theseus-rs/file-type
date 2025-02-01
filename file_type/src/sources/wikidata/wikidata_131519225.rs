use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131519225: FileFormat = FileFormat {
    id: 131_519_225,
    puid: "wikidata/131519225",
    name: "Stimulate Signal Data",
    extensions: &["sdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
