use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_15955723: FileType = FileType {
    file_format: &FileFormat {
        id: 15_955_723,
        source_type: SourceType::Wikidata,
        name: "Python script",
        extensions: &["py"],
        media_types: &[
            "application/x-httpd-python",
            "text/x-python",
            "text/x-script.python",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
