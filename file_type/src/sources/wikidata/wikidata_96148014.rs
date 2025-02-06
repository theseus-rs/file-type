use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96148014: FileFormat = FileFormat {
    id: 96_148_014,
    source_type: SourceType::Wikidata,
    name: "Wolfram Language neural net format",
    extensions: &["wlnet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
