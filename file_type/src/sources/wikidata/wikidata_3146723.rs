use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3146723: FileType = FileType {
    file_format: &FileFormat {
        id: 3_146_723,
        source_type: SourceType::Wikidata,
        name: "Raw disk image",
        extensions: &["img"],
        media_types: &["application/vnd.efi.img"],
        signatures: &[],
        related_formats: &[],
    },
};
