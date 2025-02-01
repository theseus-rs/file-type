use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117843653: FileFormat = FileFormat {
    id: 117_843_653,
    puid: "wikidata/117843653",
    name: "IBM GOCA file",
    extensions: &["gca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
