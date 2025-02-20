use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127706182: FileType = FileType {
    file_format: &FileFormat {
        id: 127_706_182,
        source_type: SourceType::Wikidata,
        name: "Less file format",
        extensions: &["less"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
