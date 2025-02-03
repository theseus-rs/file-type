use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859900: FileFormat = FileFormat {
    id: 105_859_900,
    source_type: SourceType::Wikidata,
    name: "VBIN container",
    extensions: &["vbin"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x42, 0x49, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
