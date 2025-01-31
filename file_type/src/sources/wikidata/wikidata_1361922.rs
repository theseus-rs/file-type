use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1361922: FileFormat = FileFormat {
    id: 1_361_922,
    puid: "wikidata/1361922",
    name: "netCDF",
    extensions: &["nc", "nc", "nc", "nc"],
    media_types: &[
        "application/netcdf",
        "application/netcdf",
        "application/x-netcdf",
        "application/x-netcdf",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
