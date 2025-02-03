use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2018120566: FileFormat = FileFormat {
    id: 2_018_120_566,
    source_type: SourceType::Iana,
    name: "vnd.ms-excel.sheet.binary.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-excel.sheet.binary.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
