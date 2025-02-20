use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131412758: FileType = FileType {
    file_format: &FileFormat {
        id: 131_412_758,
        source_type: SourceType::Wikidata,
        name: "VimL script file",
        extensions: &["exrc", "gvimrc", "vim", "vimrc"],
        media_types: &["text/x-vim"],
        signatures: &[],
        related_formats: &[],
    },
};
