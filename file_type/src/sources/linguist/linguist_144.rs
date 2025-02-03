use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_144: FileFormat = FileFormat {
    id: 144,
    source_type: SourceType::Linguist,
    name: "HCL",
    extensions: &["hcl", "nomad", "tf", "tfvars", "workflow"],
    media_types: &["text/x-ruby"],
    internal_signatures: &[],
    related_formats: &[],
};
