use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_94994568: FileType = FileType {
    file_format: &FileFormat {
        id: 94_994_568,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign ICML",
        extensions: &["icml"],
        media_types: &["application/x-indesign"],
        signatures: &[],
        related_formats: &[],
    },
};
