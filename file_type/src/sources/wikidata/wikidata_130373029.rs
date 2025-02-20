use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130373029: FileType = FileType {
    file_format: &FileFormat {
        id: 130_373_029,
        source_type: SourceType::Wikidata,
        name: "Newspeak file format",
        extensions: &["ns2"],
        media_types: &["text/x-newspeak"],
        signatures: &[],
        related_formats: &[],
    },
};
