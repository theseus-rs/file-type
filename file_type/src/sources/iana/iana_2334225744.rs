use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2334225744: FileType = FileType {
    file_format: &FileFormat {
        id: 2_334_225_744,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-basic-layout",
        extensions: &[],
        media_types: &["text/vnd.radisys.msml-basic-layout"],
        signatures: &[],
        related_formats: &[],
    },
};
