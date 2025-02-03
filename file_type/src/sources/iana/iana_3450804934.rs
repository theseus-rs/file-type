use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3450804934: FileFormat = FileFormat {
    id: 3_450_804_934,
    source_type: SourceType::Iana,
    name: "vnd.dolby.heaac.2",
    extensions: &[],
    media_types: &["audio/vnd.dolby.heaac.2"],
    internal_signatures: &[],
    related_formats: &[],
};
