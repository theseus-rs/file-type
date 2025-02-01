use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_292565: FileFormat = FileFormat {
    id: 292_565,
    puid: "wikidata/292565",
    name: "vCalendar",
    extensions: &["vcs"],
    media_types: &["text/x-vcalendar"],
    internal_signatures: &[],
    related_formats: &[],
};
