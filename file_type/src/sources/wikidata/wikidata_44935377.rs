use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_44935377: FileType = FileType {
    file_format: &FileFormat {
        id: 44_935_377,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel Template 97-2003",
        extensions: &["xlt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
