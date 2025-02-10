use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131303478: FileType = FileType {
    file_format: &FileFormat {
        id: 131_303_478,
        source_type: SourceType::Wikidata,
        name: "TiddlerFile",
        extensions: &["tid"],
        media_types: &["application/x-tiddler", "text/vnd.tiddlywiki"],
        signatures: &[],
        related_formats: &[],
    },
};
