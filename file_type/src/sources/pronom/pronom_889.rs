use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_889: FileType = FileType {
    file_format: &FileFormat {
        id: 889,
        source_type: SourceType::Pronom,
        name: "Microsoft Multiplan",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
