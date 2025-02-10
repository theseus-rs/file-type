use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127327574: FileType = FileType {
    file_format: &FileFormat {
        id: 127_327_574,
        source_type: SourceType::Wikidata,
        name: "Clojure file",
        extensions: &["clj"],
        media_types: &["application/x-clojure", "text/x-clojure"],
        signatures: &[],
        related_formats: &[],
    },
};
