use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112943753: FileFormat = FileFormat {
    id: 112_943_753,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 GRP file",
    extensions: &["grp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
