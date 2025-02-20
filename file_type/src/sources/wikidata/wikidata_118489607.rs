use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118489607: FileType = FileType {
    file_format: &FileFormat {
        id: 118_489_607,
        source_type: SourceType::Wikidata,
        name: "Adobe Air 2.5",
        extensions: &["air"],
        media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
