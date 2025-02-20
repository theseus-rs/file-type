use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63061396: FileType = FileType {
    file_format: &FileFormat {
        id: 63_061_396,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word Document",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
