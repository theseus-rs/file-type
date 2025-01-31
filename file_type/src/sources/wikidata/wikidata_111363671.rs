use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363671: FileFormat = FileFormat {
    id: 111_363_671,
    puid: "wikidata/111363671",
    name: "Yamaha Motif XS 'all' format",
    extensions: &["x0a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
