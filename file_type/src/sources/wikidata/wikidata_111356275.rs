use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356275: FileFormat = FileFormat {
    id: 111_356_275,
    puid: "wikidata/111356275",
    name: "Yamaha Motif ES 'waves' format",
    extensions: &["w7w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
