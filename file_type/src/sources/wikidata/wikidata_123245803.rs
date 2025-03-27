use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123245803: FileType = FileType {
    file_format: &FileFormat {
        id: 123_245_803,
        source_type: SourceType::Wikidata,
        name: "Virtual Hard Disk v2 file format",
        extensions: &["vhdx"],
        media_types: &["application/x-vhdx-disk", "application/x-virtualbox-vhdx"],
        signatures: &[],
        related_formats: &[],
    },
};
