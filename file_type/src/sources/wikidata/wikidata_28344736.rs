use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28344736: FileFormat = FileFormat {
    id: 28_344_736,
    puid: "wikidata/28344736",
    name: "Macintosh resource file",
    extensions: &["dfont", "rsrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
