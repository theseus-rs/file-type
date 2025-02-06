use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131413865: FileFormat = FileFormat {
    id: 131_413_865,
    source_type: SourceType::Wikidata,
    name: "Vyper file format",
    extensions: &["vy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
