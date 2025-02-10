use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_45028191: FileType = FileType {
    file_format: &FileFormat {
        id: 45_028_191,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Backup",
        extensions: &["xlk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
