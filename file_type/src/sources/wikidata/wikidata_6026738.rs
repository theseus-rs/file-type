use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6026738: FileType = FileType {
    file_format: &FileFormat {
        id: 6_026_738,
        source_type: SourceType::Wikidata,
        name: "PFB",
        extensions: &["pfb"],
        media_types: &["font/x-postscript-pfb"],
        signatures: &[],
        related_formats: &[],
    },
};
