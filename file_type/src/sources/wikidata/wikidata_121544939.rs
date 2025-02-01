use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121544939: FileFormat = FileFormat {
    id: 121_544_939,
    puid: "wikidata/121544939",
    name: "At Home 2011 Tax Return File",
    extensions: &["t11"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
