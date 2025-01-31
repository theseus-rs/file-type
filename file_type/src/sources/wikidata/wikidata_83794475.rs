use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83794475: FileFormat = FileFormat {
    id: 83_794_475,
    puid: "wikidata/83794475",
    name: "FO File",
    extensions: &["fo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
