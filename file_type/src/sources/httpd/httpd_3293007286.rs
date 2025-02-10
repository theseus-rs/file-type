use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3293007286: FileType = FileType {
    file_format: &FileFormat {
        id: 3_293_007_286,
        source_type: SourceType::Httpd,
        name: "font type1",
        extensions: &["pfa", "pfb", "pfm", "afm"],
        media_types: &["application/x-font-type1"],
        signatures: &[],
        related_formats: &[],
    },
};
