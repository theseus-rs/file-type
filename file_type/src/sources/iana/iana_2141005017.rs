use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2141005017: FileFormat = FileFormat {
    id: 2_141_005_017,
    source_type: SourceType::Iana,
    name: "vnd.imagemeter.folder+zip",
    extensions: &[],
    media_types: &["application/vnd.imagemeter.folder+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
