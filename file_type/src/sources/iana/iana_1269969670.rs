use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1269969670: FileFormat = FileFormat {
    id: 1_269_969_670,
    source_type: SourceType::Iana,
    name: "vnd.RenLearn.rlprint",
    extensions: &[],
    media_types: &["application/vnd.RenLearn.rlprint"],
    signatures: &[],
    related_formats: &[],
};
