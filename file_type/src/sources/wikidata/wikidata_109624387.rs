use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109624387: FileFormat = FileFormat {
    id: 109_624_387,
    puid: "wikidata/109624387",
    name: "WebPlus Essentials Templates",
    extensions: &["wpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
