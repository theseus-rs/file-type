use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131703746: FileFormat = FileFormat {
    id: 131_703_746,
    puid: "wikidata/131703746",
    name: "xRage hdf file",
    extensions: &["h5rage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
