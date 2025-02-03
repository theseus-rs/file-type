use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131840922: FileFormat = FileFormat {
    id: 131_840_922,
    source_type: SourceType::Wikidata,
    name: "POP Ocean NetCDF file format",
    extensions: &["pop.nc", "pop.ncdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
