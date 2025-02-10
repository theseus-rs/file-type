use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205908: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_908,
        source_type: SourceType::Wikidata,
        name: "Digital Imaging and Communications in Medicine file",
        extensions: &["dcm", "dic"],
        media_types: &["application/dicom"],
        signatures: &[],
        related_formats: &[],
    },
};
