use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116861020: FileFormat = FileFormat {
    id: 116_861_020,
    puid: "wikidata/116861020",
    name: "Disney Print Creations Project",
    extensions: &["dpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
