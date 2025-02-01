use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363709: FileFormat = FileFormat {
    id: 111_363_709,
    puid: "wikidata/111363709",
    name: "Yamaha Motif XF 'voices' format",
    extensions: &["x3v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
