use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951804: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_804,
        source_type: SourceType::Wikidata,
        name: "Prolog source code file",
        extensions: &["ecl", "pl", "pro", "prolog"],
        media_types: &["text/x-prolog"],
        signatures: &[],
        related_formats: &[],
    },
};
