use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51333766: FileType = FileType {
    file_format: &FileFormat {
        id: 51_333_766,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint Add-In",
        extensions: &["ppa", "ppam"],
        media_types: &["application/mspowerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
