use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207379: FileFormat = FileFormat {
    id: 28_207_379,
    puid: "wikidata/28207379",
    name: "TIFF for Fax eXtended",
    extensions: &["tfx", "tif", "tiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
