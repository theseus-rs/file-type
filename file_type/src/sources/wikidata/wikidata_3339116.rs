use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3339116: FileFormat = FileFormat {
    id: 3_339_116,
    source_type: SourceType::Wikidata,
    name: "Newick tree format",
    extensions: &["newick"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
