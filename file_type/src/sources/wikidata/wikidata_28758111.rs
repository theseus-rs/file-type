use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28758111: FileFormat = FileFormat {
    id: 28_758_111,
    puid: "wikidata/28758111",
    name: "Internet Explorer favorites",
    extensions: &["url"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
