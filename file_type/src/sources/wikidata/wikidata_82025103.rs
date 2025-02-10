use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_82025103: FileType = FileType {
    file_format: &FileFormat {
        id: 82_025_103,
        source_type: SourceType::Wikidata,
        name: "Microsoft Reader eBook annotations",
        extensions: &["ebo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
