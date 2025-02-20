use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129425911: FileType = FileType {
    file_format: &FileFormat {
        id: 129_425_911,
        source_type: SourceType::Wikidata,
        name: "Gosu template file",
        extensions: &["gst"],
        media_types: &["text/x-gosu-template"],
        signatures: &[],
        related_formats: &[],
    },
};
