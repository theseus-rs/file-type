use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2658179: FileType = FileType {
    file_format: &FileFormat {
        id: 2_658_179,
        source_type: SourceType::Wikidata,
        name: "Virtual Machine Disk",
        extensions: &["vmdk"],
        media_types: &["application/x-virtualbox-vmdk", "application/x-vmdk-disk"],
        signatures: &[],
        related_formats: &[],
    },
};
