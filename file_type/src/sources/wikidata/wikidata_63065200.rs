use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63065200: FileFormat = FileFormat {
    id: 63_065_200,
    puid: "wikidata/63065200",
    name: "HDF4",
    extensions: &["h4", "h4", "hdf", "hdf", "hdf4", "hdf4", "he4", "he4"],
    media_types: &[
        "application/x-hdf",
        "application/x-hdf",
        "application/x-hdf",
        "application/x-hdf",
        "application/x-hdf4",
        "application/x-hdf4",
        "application/x-hdf4",
        "application/x-hdf4",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
