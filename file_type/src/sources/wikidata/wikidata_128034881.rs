use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128034881: FileType = FileType {
    file_format: &FileFormat {
        id: 128_034_881,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 20",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
