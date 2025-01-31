use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96148014: FileFormat = FileFormat {
    id: 96_148_014,
    puid: "wikidata/96148014",
    name: "Wolfram Language neural net format",
    extensions: &["wlnet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
