use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333121: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_121,
        source_type: SourceType::Wikidata,
        name: "OKI MSM6376 synth chip PCM format",
        extensions: &["pcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
