use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_845986989: FileFormat = FileFormat {
    id: 845_986_989,
    source_type: SourceType::Iana,
    name: "vnd.dvb.notif-aggregate-root+xml",
    extensions: &[],
    media_types: &["application/vnd.dvb.notif-aggregate-root+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
