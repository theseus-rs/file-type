use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100344893: FileFormat = FileFormat {
    id: 100_344_893,
    puid: "wikidata/100344893",
    name: "Corel Photo House Image",
    extensions: &["cps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
