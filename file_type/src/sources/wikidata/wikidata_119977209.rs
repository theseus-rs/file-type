use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119977209: FileFormat = FileFormat {
    id: 119_977_209,
    source_type: SourceType::Wikidata,
    name: "MotionArtist Document",
    extensions: &["fmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
