use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_506780613: FileFormat = FileFormat {
    id: 506_780_613,
    source_type: SourceType::Linguist,
    name: "Nextflow",
    extensions: &["nf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
