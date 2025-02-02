use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116869095: FileFormat = FileFormat {
    id: 116_869_095,
    source_type: SourceType::Wikidata,
    name: "Summitsoft Letterhead",
    extensions: &["lth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
