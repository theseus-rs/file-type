use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1260547: FileFormat = FileFormat {
    id: 1_260_547,
    source_type: SourceType::Wikidata,
    name: "LyRiCs",
    extensions: &["lrc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
