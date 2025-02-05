use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_288256: FileFormat = FileFormat {
    id: 288_256,
    source_type: SourceType::Wikidata,
    name: "ACE",
    extensions: &["ace"],
    media_types: &["application/x-ace-compressed"],
    signatures: &[],
    related_formats: &[],
};
