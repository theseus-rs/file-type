use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4174914276: FileType = FileType {
    file_format: &FileFormat {
        id: 4_174_914_276,
        source_type: SourceType::Httpd,
        name: "simtech mindmapper",
        extensions: &["twd", "twds"],
        media_types: &["application/vnd.simtech-mindmapper"],
        signatures: &[],
        related_formats: &[],
    },
};
