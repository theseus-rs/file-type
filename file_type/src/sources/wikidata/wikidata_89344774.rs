use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_89344774: FileType = FileType {
    file_format: &FileFormat {
        id: 89_344_774,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 3.1",
        extensions: &["qwd", "qxd", "qxt"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
