use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129329858: FileFormat = FileFormat {
    id: 129_329_858,
    source_type: SourceType::Wikidata,
    name: "Genshi file",
    extensions: &["kid"],
    media_types: &["application/x-genshi", "application/x-kid"],
    signatures: &[],
    related_formats: &[],
};
