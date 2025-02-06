use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116869095: FileFormat = FileFormat {
    id: 116_869_095,
    source_type: SourceType::Wikidata,
    name: "Summitsoft Letterhead",
    extensions: &["lth"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
