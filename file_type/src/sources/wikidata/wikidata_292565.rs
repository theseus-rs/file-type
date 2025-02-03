use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_292565: FileFormat = FileFormat {
    id: 292_565,
    source_type: SourceType::Wikidata,
    name: "vCalendar",
    extensions: &["vcs"],
    media_types: &["text/x-vcalendar"],
    internal_signatures: &[],
    related_formats: &[],
};
