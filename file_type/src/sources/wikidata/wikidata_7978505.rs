use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7978505: FileFormat = FileFormat {
    id: 7_978_505,
    source_type: SourceType::Wikidata,
    name: "Web ARChive",
    extensions: &["warc"],
    media_types: &["application/warc"],
    internal_signatures: &[],
    related_formats: &[],
};
