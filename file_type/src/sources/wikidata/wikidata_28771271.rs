use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28771271: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_271,
        source_type: SourceType::Wikidata,
        name: "MSA (Magic Shadow Archiver)",
        extensions: &["msa"],
        media_types: &["application/vnd.msa-disk-image"],
        signatures: &[],
        related_formats: &[],
    },
};
