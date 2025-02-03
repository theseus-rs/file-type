use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3621718850: FileFormat = FileFormat {
    id: 3_621_718_850,
    source_type: SourceType::Iana,
    name: "mtl",
    extensions: &[],
    media_types: &["model/mtl"],
    internal_signatures: &[],
    related_formats: &[],
};
