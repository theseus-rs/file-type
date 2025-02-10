use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_91226396: FileType = FileType {
    file_format: &FileFormat {
        id: 91_226_396,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 2016",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
