use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_307400137: FileFormat = FileFormat {
    id: 307_400_137,
    source_type: SourceType::Iana,
    name: "vnd.igloader",
    extensions: &[],
    media_types: &["application/vnd.igloader"],
    internal_signatures: &[],
    related_formats: &[],
};
