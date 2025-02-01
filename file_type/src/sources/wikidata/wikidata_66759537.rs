use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66759537: FileFormat = FileFormat {
    id: 66_759_537,
    puid: "wikidata/66759537",
    name: "Excel Template",
    extensions: &["xltx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
