use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205717: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_717,
        source_type: SourceType::Wikidata,
        name: "PFS: 1st Publisher Clip Art Format",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
