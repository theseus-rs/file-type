use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111356213: FileFormat = FileFormat {
    id: 111_356_213,
    puid: "wikidata/111356213",
    name: "Yamaha Motif (6/7/8) 'waves' format",
    extensions: &["w2w"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
