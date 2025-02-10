use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50375274: FileType = FileType {
    file_format: &FileFormat {
        id: 50_375_274,
        source_type: SourceType::Wikidata,
        name: "Microsoft OneNote Package File",
        extensions: &["onepkg"],
        media_types: &["application/onenote"],
        signatures: &[],
        related_formats: &[],
    },
};
