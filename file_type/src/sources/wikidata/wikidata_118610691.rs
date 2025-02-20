use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118610691: FileType = FileType {
    file_format: &FileFormat {
        id: 118_610_691,
        source_type: SourceType::Wikidata,
        name: "Visual C++ IntelliSense Database file",
        extensions: &["ncb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
