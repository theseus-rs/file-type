use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356257: FileFormat = FileFormat {
    id: 111_356_257,
    puid: "wikidata/111356257",
    name: "Yamaha Motif ES 'all' format",
    extensions: &["w7a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
