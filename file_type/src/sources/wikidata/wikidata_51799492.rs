use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51799492: FileType = FileType {
    file_format: &FileFormat {
        id: 51_799_492,
        source_type: SourceType::Wikidata,
        name: "Quattro Pro Spreadsheet for DOS, versions 1-4",
        extensions: &["wkq", "wq1"],
        media_types: &["application/x-quattro-pro"],
        signatures: &[],
        related_formats: &[],
    },
};
