use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009492: FileFormat = FileFormat {
    id: 28_009_492,
    puid: "wikidata/28009492",
    name: "Warcraft II PUD",
    extensions: &["pud"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
