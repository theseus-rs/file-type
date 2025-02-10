use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_151241392: FileType = FileType {
    file_format: &FileFormat {
        id: 151_241_392,
        source_type: SourceType::Linguist,
        name: "Snakemake",
        extensions: &["smk", "snakefile"],
        media_types: &["text/x-python"],
        signatures: &[],
        related_formats: &[],
    },
};
