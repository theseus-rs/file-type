use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_290087393: FileType = FileType {
    file_format: &FileFormat {
        id: 290_087_393,
        source_type: SourceType::Httpd,
        name: "fujixerox docuworks",
        extensions: &["xdw"],
        media_types: &["application/vnd.fujixerox.docuworks"],
        signatures: &[],
        related_formats: &[],
    },
};
