use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50308902: FileType = FileType {
    file_format: &FileFormat {
        id: 50_308_902,
        source_type: SourceType::Wikidata,
        name: "Mobile eXtensible Music Format",
        extensions: &["mxmf"],
        media_types: &["audio/mobile-xmf"],
        signatures: &[],
        related_formats: &[],
    },
};
