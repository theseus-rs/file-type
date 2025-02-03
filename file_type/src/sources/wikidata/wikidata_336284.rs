use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_336284: FileFormat = FileFormat {
    id: 336_284,
    source_type: SourceType::Wikidata,
    name: "MPEG-1",
    extensions: &[
        "m1a", "m1v", "m2a", "mp1", "mp2", "mp3", "mpa", "mpeg", "mpg", "mpv",
    ],
    media_types: &["audio/mpeg", "video/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
