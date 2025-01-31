use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83369969: FileFormat = FileFormat {
    id: 83_369_969,
    puid: "wikidata/83369969",
    name: "FastTracker2 Extended",
    extensions: &["xi", "xi", "xm", "xm"],
    media_types: &["audio/x-xi", "audio/x-xi", "audio/x-xm", "audio/x-xm"],
    internal_signatures: &[],
    related_formats: &[],
};
