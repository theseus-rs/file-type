use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75539922: FileFormat = FileFormat {
    id: 75_539_922,
    source_type: SourceType::Wikidata,
    name: "Ulead Private Data",
    extensions: &["upd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
