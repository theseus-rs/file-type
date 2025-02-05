use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119977209: FileFormat = FileFormat {
    id: 119_977_209,
    source_type: SourceType::Wikidata,
    name: "MotionArtist Document",
    extensions: &["fmd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
