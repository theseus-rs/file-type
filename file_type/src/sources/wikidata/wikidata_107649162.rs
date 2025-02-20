use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_107649162: FileType = FileType {
    file_format: &FileFormat {
        id: 107_649_162,
        source_type: SourceType::Wikidata,
        name: "mzMLb",
        extensions: &["mzMLb"],
        media_types: &["application/x-hdf5"],
        signatures: &[],
        related_formats: &[],
    },
};
