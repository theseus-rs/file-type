use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_800983837: FileType = FileType {
    file_format: &FileFormat {
        id: 800_983_837,
        source_type: SourceType::Linguist,
        name: "Microsoft Developer Studio Project",
        extensions: &["dsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
