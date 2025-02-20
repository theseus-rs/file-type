use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_89347372: FileType = FileType {
    file_format: &FileFormat {
        id: 89_347_372,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 4",
        extensions: &["qwd", "qxd", "qxt"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
