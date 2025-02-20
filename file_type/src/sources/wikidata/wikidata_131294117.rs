use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131294117: FileType = FileType {
    file_format: &FileFormat {
        id: 131_294_117,
        source_type: SourceType::Wikidata,
        name: "Terraform file format",
        extensions: &["hcl", "tf"],
        media_types: &["application/x-terraform", "application/x-tf"],
        signatures: &[],
        related_formats: &[],
    },
};
