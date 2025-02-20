use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1167296258: FileType = FileType {
    file_format: &FileFormat {
        id: 1_167_296_258,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
