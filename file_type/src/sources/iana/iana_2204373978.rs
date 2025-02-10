use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2204373978: FileType = FileType {
    file_format: &FileFormat {
        id: 2_204_373_978,
        source_type: SourceType::Iana,
        name: "vnd.ims.imsccv1p2",
        extensions: &[],
        media_types: &["application/vnd.ims.imsccv1p2"],
        signatures: &[],
        related_formats: &[],
    },
};
