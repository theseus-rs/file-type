use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63098174: FileType = FileType {
    file_format: &FileFormat {
        id: 63_098_174,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visio XML Drawing, version 2003-2010",
        extensions: &["vdx"],
        media_types: &["application/vnd.visio"],
        signatures: &[],
        related_formats: &[],
    },
};
