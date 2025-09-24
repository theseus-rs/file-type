use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167920: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_920,
        source_type: SourceType::Wikidata,
        name: "PEA",
        extensions: &["pea"],
        media_types: &[
            "http://www.wikidata.org/.well-known/genid/514d4d28088ff52a4a87915b92950d02",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
