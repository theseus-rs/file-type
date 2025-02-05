use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127699802: FileFormat = FileFormat {
    id: 127_699_802,
    source_type: SourceType::Wikidata,
    name: "Futhark file",
    extensions: &["fut"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
