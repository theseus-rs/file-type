use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134468895: FileType = FileType {
    file_format: &FileFormat {
        id: 134_468_895,
        source_type: SourceType::Wikidata,
        name: "Microsoft Macro Assembler file",
        extensions: &["masm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
