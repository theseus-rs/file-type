use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2865871100: FileType = FileType {
    file_format: &FileFormat {
        id: 2_865_871_100,
        source_type: SourceType::Iana,
        name: "vnd.sap.vds",
        extensions: &[],
        media_types: &["model/vnd.sap.vds"],
        signatures: &[],
        related_formats: &[],
    },
};
