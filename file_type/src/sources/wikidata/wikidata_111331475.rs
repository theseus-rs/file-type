use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111331475: FileFormat = FileFormat {
    id: 111_331_475,
    puid: "wikidata/111331475",
    name: "Mus10 audio file",
    extensions: &["mus10"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
