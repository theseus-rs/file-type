use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960099: FileFormat = FileFormat {
    id: 27_960_099,
    puid: "wikidata/27960099",
    name: "Stems",
    extensions: &["stem.mp4"],
    media_types: &["video/audio"],
    internal_signatures: &[],
    related_formats: &[],
};
