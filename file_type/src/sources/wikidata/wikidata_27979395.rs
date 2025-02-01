use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979395: FileFormat = FileFormat {
    id: 27_979_395,
    puid: "wikidata/27979395",
    name: "GIFV",
    extensions: &["gifv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
