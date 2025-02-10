use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121760718: FileType = FileType {
    file_format: &FileFormat {
        id: 121_760_718,
        source_type: SourceType::Wikidata,
        name: "Adobe Acrobat MIME Encoded Job Definition File",
        extensions: &["mjd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
