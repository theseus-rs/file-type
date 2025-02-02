use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110995712: FileFormat = FileFormat {
    id: 110_995_712,
    source_type: SourceType::Wikidata,
    name: "VideoWave Production File",
    extensions: &["sbd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
