use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111263087: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_087,
        source_type: SourceType::Wikidata,
        name: "Yamaha DX100/DX21/DX27 voice SysEx dump",
        extensions: &["bn4", "syx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
