use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61080677: FileFormat = FileFormat {
    id: 61_080_677,
    puid: "wikidata/61080677",
    name: "HDF5",
    extensions: &["h5", "hdf", "hdf5", "he5"],
    media_types: &[
        "application/x-hdf5",
        "application/x-hdf5",
        "application/x-hdf5",
        "application/x-hdf5",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
