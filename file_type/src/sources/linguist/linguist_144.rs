use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_144: FileType = FileType {
    file_format: &FileFormat {
        id: 144,
        source_type: SourceType::Linguist,
        name: "HCL",
        extensions: &["hcl", "nomad", "tf", "tfvars", "workflow"],
        media_types: &["text/x-ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
