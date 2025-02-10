use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344823: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_823,
        source_type: SourceType::Wikidata,
        name: "Debian source control file",
        extensions: &["dsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
