use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967421: FileFormat = FileFormat {
    id: 27_967_421,
    source_type: SourceType::Wikidata,
    name: "CapXML",
    extensions: &["capx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
