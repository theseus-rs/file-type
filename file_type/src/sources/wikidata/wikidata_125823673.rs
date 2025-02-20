use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125823673: FileType = FileType {
    file_format: &FileFormat {
        id: 125_823_673,
        source_type: SourceType::Wikidata,
        name: "Gzipped Tar File",
        extensions: &["tgz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
