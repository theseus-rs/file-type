use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131703980: FileFormat = FileFormat {
    id: 131_703_980,
    puid: "wikidata/131703980",
    name: "GE TRUCHAS file format",
    extensions: &["h5", "hdf5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
