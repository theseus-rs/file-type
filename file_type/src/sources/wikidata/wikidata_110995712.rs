use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110995712: FileFormat = FileFormat {
    id: 110_995_712,
    source_type: SourceType::Wikidata,
    name: "VideoWave Production File",
    extensions: &["sbd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
