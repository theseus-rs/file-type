use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1260547: FileFormat = FileFormat {
    id: 1_260_547,
    source_type: SourceType::Wikidata,
    name: "LyRiCs",
    extensions: &["lrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
