use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117843750: FileFormat = FileFormat {
    id: 117_843_750,
    puid: "wikidata/117843750",
    name: "IBM IOCA",
    extensions: &["ica"],
    media_types: &["image/x-ioca"],
    internal_signatures: &[],
    related_formats: &[],
};
