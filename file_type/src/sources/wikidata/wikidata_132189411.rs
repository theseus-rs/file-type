use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132189411: FileType = FileType {
    file_format: &FileFormat {
        id: 132_189_411,
        source_type: SourceType::Wikidata,
        name: "Sao Mai Braille file format",
        extensions: &["smb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
