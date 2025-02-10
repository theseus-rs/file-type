use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28807546: FileType = FileType {
    file_format: &FileFormat {
        id: 28_807_546,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Binder File for Windows 97-2000",
        extensions: &["obd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
