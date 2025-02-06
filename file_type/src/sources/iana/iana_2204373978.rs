use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2204373978: FileFormat = FileFormat {
    id: 2_204_373_978,
    source_type: SourceType::Iana,
    name: "vnd.ims.imsccv1p2",
    extensions: &[],
    media_types: &["application/vnd.ims.imsccv1p2"],
    signatures: &[],
    related_formats: &[],
};
