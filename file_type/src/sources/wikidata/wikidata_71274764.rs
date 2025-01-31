use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71274764: FileFormat = FileFormat {
    id: 71_274_764,
    puid: "wikidata/71274764",
    name: "CorelDraw Template",
    extensions: &["cdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
