use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3386406444: FileFormat = FileFormat {
    id: 3_386_406_444,
    source_type: SourceType::Iana,
    name: "matroska-3d",
    extensions: &[],
    media_types: &["video/matroska-3d"],
    internal_signatures: &[],
    related_formats: &[],
};
