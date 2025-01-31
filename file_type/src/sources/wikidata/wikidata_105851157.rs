use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851157: FileFormat = FileFormat {
    id: 105_851_157,
    puid: "wikidata/105851157",
    name: "World of Warcraft TOC file",
    extensions: &["toc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
