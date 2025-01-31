use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_901031: FileFormat = FileFormat {
    id: 901_031,
    puid: "wikidata/901031",
    name: "device independent file format",
    extensions: &["dvi"],
    media_types: &["application/x-dvi"],
    internal_signatures: &[],
    related_formats: &[],
};
