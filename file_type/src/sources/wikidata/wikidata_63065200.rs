use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63065200: FileFormat = FileFormat {
    id: 63_065_200,
    source_type: SourceType::Wikidata,
    name: "HDF4",
    extensions: &["h4", "hdf", "hdf4", "he4"],
    media_types: &["application/x-hdf", "application/x-hdf4"],
    internal_signatures: &[],
    related_formats: &[],
};
