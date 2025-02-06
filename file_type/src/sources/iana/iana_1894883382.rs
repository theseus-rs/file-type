use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1894883382: FileFormat = FileFormat {
    id: 1_894_883_382,
    source_type: SourceType::Iana,
    name: "vnd.erofs",
    extensions: &[],
    media_types: &["application/vnd.erofs"],
    signatures: &[],
    related_formats: &[],
};
