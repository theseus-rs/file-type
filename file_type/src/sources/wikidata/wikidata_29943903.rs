use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29943903: FileType = FileType {
    file_format: &FileFormat {
        id: 29_943_903,
        source_type: SourceType::Wikidata,
        name: "StarOffice Writer, version 3.x",
        extensions: &["sdw"],
        media_types: &[
            "application/vnd.stardivision.writer",
            "application/x-starwriter",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
