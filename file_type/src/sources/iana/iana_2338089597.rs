use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2338089597: FileFormat = FileFormat {
    id: 2_338_089_597,
    source_type: SourceType::Iana,
    name: "scip",
    extensions: &[],
    media_types: &["video/scip"],
    internal_signatures: &[],
    related_formats: &[],
};
