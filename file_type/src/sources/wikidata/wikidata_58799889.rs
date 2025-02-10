use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58799889: FileType = FileType {
    file_format: &FileFormat {
        id: 58_799_889,
        source_type: SourceType::Wikidata,
        name: "PowerProject Teamplan",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
