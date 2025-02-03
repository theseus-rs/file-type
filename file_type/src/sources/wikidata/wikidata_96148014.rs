use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96148014: FileFormat = FileFormat {
    id: 96_148_014,
    source_type: SourceType::Wikidata,
    name: "Wolfram Language neural net format",
    extensions: &["wlnet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
