use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363686: FileFormat = FileFormat {
    id: 111_363_686,
    puid: "wikidata/111363686",
    name: "Yamaha Motif XS 'voices' format",
    extensions: &["x0v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
