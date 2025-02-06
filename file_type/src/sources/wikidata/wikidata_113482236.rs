use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113482236: FileFormat = FileFormat {
    id: 113_482_236,
    source_type: SourceType::Wikidata,
    name: "602 Graph/Chart File 1.51",
    extensions: &["gc6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
