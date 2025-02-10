use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
