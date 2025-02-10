use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47524799: FileType = FileType {
    file_format: &FileFormat {
        id: 47_524_799,
        source_type: SourceType::Wikidata,
        name: "Quark Xpress Data File, version 9",
        extensions: &["qcd", "qct", "qpt", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
