use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967127: FileFormat = FileFormat {
    id: 27_967_127,
    source_type: SourceType::Wikidata,
    name: "CMS",
    extensions: &["cms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
