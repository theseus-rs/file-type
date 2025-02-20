use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115923522: FileType = FileType {
    file_format: &FileFormat {
        id: 115_923_522,
        source_type: SourceType::Wikidata,
        name: "HCL configuration file",
        extensions: &["hcl", "tf"],
        media_types: &["text/x-hcl"],
        signatures: &[],
        related_formats: &[],
    },
};
