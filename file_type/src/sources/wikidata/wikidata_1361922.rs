use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1361922: FileType = FileType {
    file_format: &FileFormat {
        id: 1_361_922,
        source_type: SourceType::Wikidata,
        name: "netCDF",
        extensions: &["nc"],
        media_types: &["application/netcdf", "application/x-netcdf"],
        signatures: &[],
        related_formats: &[],
    },
};
