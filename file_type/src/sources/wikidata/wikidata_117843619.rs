use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117843619: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_619,
        source_type: SourceType::Wikidata,
        name: "Gammalink Gamma Fax file",
        extensions: &["gam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
