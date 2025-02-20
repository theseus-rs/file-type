use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130981140: FileType = FileType {
    file_format: &FileFormat {
        id: 130_981_140,
        source_type: SourceType::Wikidata,
        name: "Slurm script file format",
        extensions: &["sl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
