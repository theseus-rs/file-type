use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34742387: FileType = FileType {
    file_format: &FileFormat {
        id: 34_742_387,
        source_type: SourceType::Wikidata,
        name: "Softdisk Family Tree 2 Marriage Data",
        extensions: &["ftm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
