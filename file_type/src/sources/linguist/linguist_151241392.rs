use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_151241392: FileFormat = FileFormat {
    id: 151_241_392,
    source_type: SourceType::Linguist,
    name: "Snakemake",
    extensions: &["smk", "snakefile"],
    media_types: &["text/x-python"],
    internal_signatures: &[],
    related_formats: &[],
};
