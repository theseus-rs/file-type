use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_564186416: FileType = FileType {
    file_format: &FileFormat {
        id: 564_186_416,
        source_type: SourceType::Linguist,
        name: "ASP.NET",
        extensions: &["asax", "ascx", "ashx", "asmx", "aspx", "axd"],
        media_types: &["application/x-aspx"],
        signatures: &[],
        related_formats: &[],
    },
};
