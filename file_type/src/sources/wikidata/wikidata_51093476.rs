use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51093476: FileFormat = FileFormat {
    id: 51_093_476,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel OLAP Query",
    extensions: &["oqy"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
