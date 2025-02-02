use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5227450111622812231: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ibm modcap",
    extensions: &["afp", "listafp", "list3820"],
    media_types: &["application/vnd.ibm.modcap"],
    internal_signatures: &[],
    related_formats: &[],
};
