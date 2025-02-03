use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1215761775: FileFormat = FileFormat {
    id: 1_215_761_775,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.pic-bw-large",
    extensions: &[],
    media_types: &["application/vnd.3gpp.pic-bw-large"],
    internal_signatures: &[],
    related_formats: &[],
};
