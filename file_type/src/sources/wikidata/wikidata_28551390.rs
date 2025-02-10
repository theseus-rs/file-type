use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551390: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_390,
        source_type: SourceType::Wikidata,
        name: "Adobe Selective Color File",
        extensions: &["asv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
