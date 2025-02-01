use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979332: FileFormat = FileFormat {
    id: 27_979_332,
    puid: "wikidata/27979332",
    name: "Age of Mythology BAR format",
    extensions: &["bar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
