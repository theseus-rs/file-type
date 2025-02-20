use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96054624: FileType = FileType {
    file_format: &FileFormat {
        id: 96_054_624,
        source_type: SourceType::Wikidata,
        name: "Modelica model format",
        extensions: &["mo"],
        media_types: &["text/x-modelica"],
        signatures: &[],
        related_formats: &[],
    },
};
