use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960099: FileFormat = FileFormat {
    id: 27_960_099,
    source_type: SourceType::Wikidata,
    name: "Stems",
    extensions: &["stem.mp4"],
    media_types: &["video/audio"],
    internal_signatures: &[],
    related_formats: &[],
};
