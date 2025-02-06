use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29896310: FileFormat = FileFormat {
    id: 29_896_310,
    source_type: SourceType::Wikidata,
    name: "ABIF",
    extensions: &["ab1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
