use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51093445: FileType = FileType {
    file_format: &FileFormat {
        id: 51_093_445,
        source_type: SourceType::Wikidata,
        name: "Microsoft Outlook Address Book",
        extensions: &["olk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
