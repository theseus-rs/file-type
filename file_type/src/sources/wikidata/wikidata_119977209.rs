use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119977209: FileFormat = FileFormat {
    id: 119_977_209,
    puid: "wikidata/119977209",
    name: "MotionArtist Document",
    extensions: &["fmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
