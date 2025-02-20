use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67451099: FileType = FileType {
    file_format: &FileFormat {
        id: 67_451_099,
        source_type: SourceType::Wikidata,
        name: "IBM Softcopy Reader (Bookmanager) Bookshelf (and Book) index file",
        extensions: &["bki"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
