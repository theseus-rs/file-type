use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1035892117: FileType = FileType {
    file_format: &FileFormat {
        id: 1_035_892_117,
        source_type: SourceType::Linguist,
        name: "TSV",
        extensions: &["tsv", "vcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
