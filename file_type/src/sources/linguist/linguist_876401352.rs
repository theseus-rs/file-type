use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_876401352: FileType = FileType {
    file_format: &FileFormat {
        id: 876_401_352,
        source_type: SourceType::Linguist,
        name: "GtkRC",
        extensions: &["gtkrc"],
        media_types: &["text/x-ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
