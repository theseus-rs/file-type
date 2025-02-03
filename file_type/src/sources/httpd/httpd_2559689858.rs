use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2559689858: FileFormat = FileFormat {
    id: 2_559_689_858,
    source_type: SourceType::Httpd,
    name: "fujitsu oasysgp",
    extensions: &["fg5"],
    media_types: &["application/vnd.fujitsu.oasysgp"],
    internal_signatures: &[],
    related_formats: &[],
};
