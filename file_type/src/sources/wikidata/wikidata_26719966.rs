use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26719966: FileType = FileType {
    file_format: &FileFormat {
        id: 26_719_966,
        source_type: SourceType::Wikidata,
        name: "Computer Graphics Metafile, version 1",
        extensions: &["cgm"],
        media_types: &["image/cgm"],
        signatures: &[],
        related_formats: &[],
    },
};
