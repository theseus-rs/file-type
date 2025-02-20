use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_731135: FileType = FileType {
    file_format: &FileFormat {
        id: 731_135,
        source_type: SourceType::Wikidata,
        name: "Microsoft Management Console",
        extensions: &["msc"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
