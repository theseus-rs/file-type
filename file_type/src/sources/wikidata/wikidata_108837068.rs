use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837068: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_068,
        source_type: SourceType::Wikidata,
        name: "Nero UDF CD-ROM Compilation CD-Image",
        extensions: &["nrg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
