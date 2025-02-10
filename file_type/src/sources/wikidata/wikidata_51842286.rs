use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51842286: FileType = FileType {
    file_format: &FileFormat {
        id: 51_842_286,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint Graphics File",
        extensions: &["ppi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
