use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_919373872: FileType = FileType {
    file_format: &FileFormat {
        id: 919_373_872,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-dialog-fax-detect+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-dialog-fax-detect+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
