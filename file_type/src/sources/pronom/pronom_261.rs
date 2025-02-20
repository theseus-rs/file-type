use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_261: FileType = FileType {
    file_format: &FileFormat {
        id: 261,
        source_type: SourceType::Pronom,
        name: "SDSC Image Tool Wavefront Raster Image",
        extensions: &["rla"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
