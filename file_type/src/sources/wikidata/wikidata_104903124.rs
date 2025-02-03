use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104903124: FileFormat = FileFormat {
    id: 104_903_124,
    source_type: SourceType::Wikidata,
    name: "Web Archive Collection Zipped",
    extensions: &["wacz"],
    media_types: &["application/x-wacz"],
    internal_signatures: &[],
    related_formats: &[],
};
