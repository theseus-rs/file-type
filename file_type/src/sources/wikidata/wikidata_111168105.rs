use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111168105: FileFormat = FileFormat {
    id: 111_168_105,
    source_type: SourceType::Wikidata,
    name: "ChemSketch 2.0 Document",
    extensions: &["sk2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
