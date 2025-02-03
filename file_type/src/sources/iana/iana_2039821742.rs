use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2039821742: FileFormat = FileFormat {
    id: 2_039_821_742,
    source_type: SourceType::Iana,
    name: "cwl+yaml",
    extensions: &[],
    media_types: &["application/cwl+yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
