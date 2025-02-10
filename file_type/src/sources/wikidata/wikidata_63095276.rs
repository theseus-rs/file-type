use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63095276: FileType = FileType {
    file_format: &FileFormat {
        id: 63_095_276,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint for Windows file format",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
