use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29943240: FileType = FileType {
    file_format: &FileFormat {
        id: 29_943_240,
        source_type: SourceType::Wikidata,
        name: "StarOffice Calc, version 3.x",
        extensions: &["sdc"],
        media_types: &[
            "application/vnd.stardivision.calc",
            "application/x-starcalc",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
