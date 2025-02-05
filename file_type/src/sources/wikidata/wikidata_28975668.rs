use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975668: FileFormat = FileFormat {
    id: 28_975_668,
    source_type: SourceType::Wikidata,
    name: "Alchemy 2000 Molecule format",
    extensions: &["al2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
