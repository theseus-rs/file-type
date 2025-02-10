use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66759447: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_447,
        source_type: SourceType::Wikidata,
        name: "Microsoft Office Access Signed Packages",
        extensions: &["accdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
