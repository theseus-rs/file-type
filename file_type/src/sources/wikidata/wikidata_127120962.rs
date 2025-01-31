use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127120962: FileFormat = FileFormat {
    id: 127_120_962,
    puid: "wikidata/127120962",
    name: "Matrix Market file format",
    extensions: &["mm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
