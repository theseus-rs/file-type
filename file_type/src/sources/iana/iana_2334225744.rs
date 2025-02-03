use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2334225744: FileFormat = FileFormat {
    id: 2_334_225_744,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-basic-layout",
    extensions: &[],
    media_types: &["text/vnd.radisys.msml-basic-layout"],
    internal_signatures: &[],
    related_formats: &[],
};
