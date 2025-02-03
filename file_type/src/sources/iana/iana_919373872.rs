use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_919373872: FileFormat = FileFormat {
    id: 919_373_872,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-dialog-fax-detect+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-dialog-fax-detect+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
