use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61080677: FileFormat = FileFormat {
    id: 61_080_677,
    source_type: SourceType::Wikidata,
    name: "HDF5",
    extensions: &["h5", "hdf", "hdf5", "he5"],
    media_types: &["application/x-hdf5"],
    internal_signatures: &[],
    related_formats: &[],
};
