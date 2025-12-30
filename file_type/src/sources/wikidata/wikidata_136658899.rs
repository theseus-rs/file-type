use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136658899: FileType = FileType {
    file_format: &FileFormat {
        id: 136_658_899,
        source_type: SourceType::Wikidata,
        name: "Not Quite Perl file",
        extensions: &["nqp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
