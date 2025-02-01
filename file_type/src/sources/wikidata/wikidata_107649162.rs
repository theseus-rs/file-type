use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_107649162: FileFormat = FileFormat {
    id: 107_649_162,
    puid: "wikidata/107649162",
    name: "mzMLb",
    extensions: &["mzMLb"],
    media_types: &["application/x-hdf5"],
    internal_signatures: &[],
    related_formats: &[],
};
