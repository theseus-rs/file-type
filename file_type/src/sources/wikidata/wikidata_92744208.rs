use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_92744208: FileType = FileType {
    file_format: &FileFormat {
        id: 92_744_208,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 2019",
        extensions: &["qpt", "qwd", "qxp"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
