use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356268: FileFormat = FileFormat {
    id: 111_356_268,
    puid: "wikidata/111356268",
    name: "Yamaha Motif ES 'voices' format",
    extensions: &["w7v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
