use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48782394: FileType = FileType {
    file_format: &FileFormat {
        id: 48_782_394,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for MS-DOS Document, version 5.5",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
