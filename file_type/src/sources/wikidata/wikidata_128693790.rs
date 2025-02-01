use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693790: FileFormat = FileFormat {
    id: 128_693_790,
    puid: "wikidata/128693790",
    name: "Boa file format",
    extensions: &["boa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
