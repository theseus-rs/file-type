use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58725633: FileType = FileType {
    file_format: &FileFormat {
        id: 58_725_633,
        source_type: SourceType::Wikidata,
        name: "Adobe PostScript Font Metrics file",
        extensions: &["pfm"],
        media_types: &["application/x-font-pfm"],
        signatures: &[],
        related_formats: &[],
    },
};
