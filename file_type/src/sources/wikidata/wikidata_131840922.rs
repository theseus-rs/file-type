use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131840922: FileType = FileType {
    file_format: &FileFormat {
        id: 131_840_922,
        source_type: SourceType::Wikidata,
        name: "POP Ocean NetCDF file format",
        extensions: &["pop.nc", "pop.ncdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
