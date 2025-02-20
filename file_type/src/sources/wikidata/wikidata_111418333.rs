use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111418333: FileType = FileType {
    file_format: &FileFormat {
        id: 111_418_333,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge URL File",
        extensions: &["adobebridge"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
