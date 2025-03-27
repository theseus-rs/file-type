use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34742774: FileType = FileType {
    file_format: &FileFormat {
        id: 34_742_774,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 2 Place Name Data",
        extensions: &["ftp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
