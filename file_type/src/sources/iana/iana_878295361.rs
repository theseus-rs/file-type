use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_878295361: FileFormat = FileFormat {
    id: 878_295_361,
    source_type: SourceType::Iana,
    name: "vnd.desmume.movie",
    extensions: &[],
    media_types: &["application/vnd.desmume.movie"],
    internal_signatures: &[],
    related_formats: &[],
};
