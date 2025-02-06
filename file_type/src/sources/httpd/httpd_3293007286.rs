use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3293007286: FileFormat = FileFormat {
    id: 3_293_007_286,
    source_type: SourceType::Httpd,
    name: "font type1",
    extensions: &["pfa", "pfb", "pfm", "afm"],
    media_types: &["application/x-font-type1"],
    signatures: &[],
    related_formats: &[],
};
