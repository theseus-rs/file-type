use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355673: FileFormat = FileFormat {
    id: 111_355_673,
    puid: "wikidata/111355673",
    name: "Yamaha Motif (6/7/8) 'all' format",
    extensions: &["w2a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
