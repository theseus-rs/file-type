use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3978310969: FileFormat = FileFormat {
    id: 3_978_310_969,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcvideo-user-profile+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcvideo-user-profile+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
