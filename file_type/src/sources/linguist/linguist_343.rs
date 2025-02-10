use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_343: FileType = FileType {
    file_format: &FileFormat {
        id: 343,
        source_type: SourceType::Linguist,
        name: "Scheme",
        extensions: &["sch", "scm", "sld", "sls", "sps", "ss"],
        media_types: &["text/x-scheme"],
        signatures: &[],
        related_formats: &[],
    },
};
