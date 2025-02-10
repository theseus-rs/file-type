use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61707607: FileType = FileType {
    file_format: &FileFormat {
        id: 61_707_607,
        source_type: SourceType::Wikidata,
        name: "Microsoft Outlook Email Message",
        extensions: &["msg", "oft"],
        media_types: &["application/vnd.ms-outlook"],
        signatures: &[],
        related_formats: &[],
    },
};
