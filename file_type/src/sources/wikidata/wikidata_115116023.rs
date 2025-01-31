use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_115116023: FileFormat = FileFormat {
    id: 115_116_023,
    puid: "wikidata/115116023",
    name: "Funpaint Image File",
    extensions: &["fp2", "fun", "vic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
