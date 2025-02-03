use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50288102: FileFormat = FileFormat {
    id: 50_288_102,
    source_type: SourceType::Wikidata,
    name: "Mathcad Document, binary file format",
    extensions: &["mcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
