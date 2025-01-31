use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27595621: FileFormat = FileFormat {
    id: 27_595_621,
    puid: "wikidata/27595621",
    name: "Audio Video Interleave with OpenDML Extensions, version 1.02",
    extensions: &["avi"],
    media_types: &["video/vnd.avi"],
    internal_signatures: &[],
    related_formats: &[],
};
