use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138491690: FileType = FileType {
    file_format: &FileFormat {
        id: 138_491_690,
        source_type: SourceType::Wikidata,
        name: "FieldWorks Language Explorer FWBackup",
        extensions: &["fwbackup"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
