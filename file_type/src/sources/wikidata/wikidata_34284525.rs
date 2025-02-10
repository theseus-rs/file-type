use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34284525: FileType = FileType {
    file_format: &FileFormat {
        id: 34_284_525,
        source_type: SourceType::Wikidata,
        name: "Perl script",
        extensions: &["pl"],
        media_types: &[
            "application/x-httpd-perl",
            "application/x-perl",
            "text/x-perl",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
