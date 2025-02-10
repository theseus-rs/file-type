use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51913355: FileType = FileType {
    file_format: &FileFormat {
        id: 51_913_355,
        source_type: SourceType::Wikidata,
        name: "Quark Xpress Data File",
        extensions: &["qcd", "qxd", "qxl", "qxp", "qxt"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
