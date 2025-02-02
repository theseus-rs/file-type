use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975668: FileFormat = FileFormat {
    id: 28_975_668,
    source_type: SourceType::Wikidata,
    name: "Alchemy 2000 Molecule format",
    extensions: &["al2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
