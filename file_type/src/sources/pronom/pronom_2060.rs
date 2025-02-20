use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2060: FileType = FileType {
    file_format: &FileFormat {
        id: 2_060,
        source_type: SourceType::Pronom,
        name: "ZFO (Form) File",
        extensions: &["zfo"],
        media_types: &["application/vnd.software602.filler.form-xml-zip"],
        signatures: &[],
        related_formats: &[],
    },
};
