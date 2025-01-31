use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48937952: FileFormat = FileFormat {
    id: 48_937_952,
    puid: "wikidata/48937952",
    name: "descript.ion",
    extensions: &["ion"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
