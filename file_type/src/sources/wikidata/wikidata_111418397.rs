use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111418397: FileFormat = FileFormat {
    id: 111_418_397,
    puid: "wikidata/111418397",
    name: "Adobe Bridge Cache Export File",
    extensions: &["bridgecache"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
