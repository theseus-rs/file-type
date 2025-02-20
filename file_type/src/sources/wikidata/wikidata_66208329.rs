use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66208329: FileType = FileType {
    file_format: &FileFormat {
        id: 66_208_329,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works communications script file",
        extensions: &["wcm"],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
