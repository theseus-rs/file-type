use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551337: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_337,
        source_type: SourceType::Wikidata,
        name: "Adobe Duotone Options File",
        extensions: &["ado"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
