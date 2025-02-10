use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28049655: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_655,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Device-Independent Binary Plotter File",
        extensions: &["adi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
