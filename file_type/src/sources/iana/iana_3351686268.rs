use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3351686268: FileFormat = FileFormat {
    id: 3_351_686_268,
    source_type: SourceType::Iana,
    name: "vnd.japannet-setstore-wakeup",
    extensions: &[],
    media_types: &["application/vnd.japannet-setstore-wakeup"],
    signatures: &[],
    related_formats: &[],
};
