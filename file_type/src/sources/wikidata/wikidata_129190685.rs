use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129190685: FileType = FileType {
    file_format: &FileFormat {
        id: 129_190_685,
        source_type: SourceType::Wikidata,
        name: "FStar file format",
        extensions: &["fst"],
        media_types: &["text/x-fstar"],
        signatures: &[],
        related_formats: &[],
    },
};
