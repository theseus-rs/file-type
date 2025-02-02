use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131413865: FileFormat = FileFormat {
    id: 131_413_865,
    source_type: SourceType::Wikidata,
    name: "Vyper file format",
    extensions: &["vy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
