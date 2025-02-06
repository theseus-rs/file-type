use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50288102: FileFormat = FileFormat {
    id: 50_288_102,
    source_type: SourceType::Wikidata,
    name: "Mathcad Document, binary file format",
    extensions: &["mcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
