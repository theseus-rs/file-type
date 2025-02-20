use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1593782: FileType = FileType {
    file_format: &FileFormat {
        id: 1_593_782,
        source_type: SourceType::Wikidata,
        name: "FASTA format",
        extensions: &["fa", "fasta"],
        media_types: &[
            "chemical/seq-aa-fasta",
            "chemical/seq-na-fasta",
            "text/plain",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
