use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117485673: FileFormat = FileFormat {
    id: 117_485_673,
    source_type: SourceType::Wikidata,
    name: "Audacity Project File (Early)",
    extensions: &["aup"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
