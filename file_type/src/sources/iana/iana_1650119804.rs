use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1650119804: FileFormat = FileFormat {
    id: 1_650_119_804,
    source_type: SourceType::Iana,
    name: "vnd.ims.imsccv1p1",
    extensions: &[],
    media_types: &["application/vnd.ims.imsccv1p1"],
    signatures: &[],
    related_formats: &[],
};
