use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860713: FileFormat = FileFormat {
    id: 105_860_713,
    source_type: SourceType::Wikidata,
    name: "RepliGo virtual print",
    extensions: &["rgo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x72, 0x67, 0x6F, 0x43, 0x45, 0x52, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
