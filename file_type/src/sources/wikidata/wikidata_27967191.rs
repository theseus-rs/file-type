use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967191: FileFormat = FileFormat {
    id: 27_967_191,
    puid: "wikidata/27967191",
    name: "Grave Composer module",
    extensions: &["wow"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
