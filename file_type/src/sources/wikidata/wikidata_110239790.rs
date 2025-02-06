use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110239790: FileFormat = FileFormat {
    id: 110_239_790,
    source_type: SourceType::Wikidata,
    name: "JData",
    extensions: &["jdb", "jdt"],
    media_types: &["application/jdata-binary", "application/jdata-text"],
    signatures: &[],
    related_formats: &[],
};
