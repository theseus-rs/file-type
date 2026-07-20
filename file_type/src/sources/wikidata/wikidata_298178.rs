use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_298178: FileType = FileType {
    file_format: &FileFormat {
        id: 298_178,
        source_type: SourceType::Wikidata,
        name: "Advanced Stream Redirector",
        extensions: &["asx"],
        media_types: &[
            "application/x-mplayer2",
            "application/x-ms-asx",
            "video/x-la-asf",
            "video/x-ms-asf",
            "video/x-ms-asf-plugin",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
