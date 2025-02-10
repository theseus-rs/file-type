use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129571777: FileType = FileType {
    file_format: &FileFormat {
        id: 129_571_777,
        source_type: SourceType::Wikidata,
        name: "Hybris source code file",
        extensions: &["hyb"],
        media_types: &["application/x-hybris", "text/x-hybris"],
        signatures: &[],
        related_formats: &[],
    },
};
