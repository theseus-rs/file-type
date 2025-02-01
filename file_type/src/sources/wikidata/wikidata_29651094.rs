use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651094: FileFormat = FileFormat {
    id: 29_651_094,
    puid: "wikidata/29651094",
    name: "ULTRA Compressed Archive",
    extensions: &["uca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
