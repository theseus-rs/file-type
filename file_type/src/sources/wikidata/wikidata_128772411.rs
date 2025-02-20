use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128772411: FileType = FileType {
    file_format: &FileFormat {
        id: 128_772_411,
        source_type: SourceType::Wikidata,
        name: "ClojureScript file format",
        extensions: &["cljs"],
        media_types: &["application/x-clojurescript", "text/x-clojurescript"],
        signatures: &[],
        related_formats: &[],
    },
};
