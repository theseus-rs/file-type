use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_64859082: FileFormat = FileFormat {
    id: 64_859_082,
    puid: "wikidata/64859082",
    name: "Family Tree Maker for DOS file format",
    extensions: &["ftm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
