use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1361922: FileFormat = FileFormat {
    id: 1_361_922,
    source_type: SourceType::Wikidata,
    name: "netCDF",
    extensions: &["nc"],
    media_types: &["application/netcdf", "application/x-netcdf"],
    internal_signatures: &[],
    related_formats: &[],
};
