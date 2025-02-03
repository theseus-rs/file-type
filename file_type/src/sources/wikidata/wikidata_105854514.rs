use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854514: FileFormat = FileFormat {
    id: 105_854_514,
    source_type: SourceType::Wikidata,
    name: "MP3 audio (ID3 v1.x tag)",
    extensions: &["mp3"],
    media_types: &["audio/mpeg3"],
    internal_signatures: &[],
    related_formats: &[],
};
