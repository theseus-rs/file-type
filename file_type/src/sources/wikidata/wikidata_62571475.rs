use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62571475: FileFormat = FileFormat {
    id: 62_571_475,
    puid: "wikidata/62571475",
    name: "Online Description Tool Format",
    extensions: &["odt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
