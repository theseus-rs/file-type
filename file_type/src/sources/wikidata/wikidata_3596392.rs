use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3596392: FileType = FileType {
    file_format: &FileFormat {
        id: 3_596_392,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word document template",
        extensions: &["dot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
