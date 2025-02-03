use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51093476: FileFormat = FileFormat {
    id: 51_093_476,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel OLAP Query",
    extensions: &["oqy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
