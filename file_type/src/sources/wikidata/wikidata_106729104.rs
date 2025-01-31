use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_106729104: FileFormat = FileFormat {
    id: 106_729_104,
    puid: "wikidata/106729104",
    name: "mz5",
    extensions: &["mz5"],
    media_types: &["application/x-hdf5"],
    internal_signatures: &[],
    related_formats: &[],
};
