use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2829476850: FileFormat = FileFormat {
    id: 2_829_476_850,
    source_type: SourceType::Iana,
    name: "vnd.si.simp (OBSOLETED by request)",
    extensions: &[],
    media_types: &["message/vnd.si.simp"],
    internal_signatures: &[],
    related_formats: &[],
};
