use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855137: FileFormat = FileFormat {
    id: 105_855_137,
    source_type: SourceType::Wikidata,
    name: "FOnline Engine GUI Scheme",
    extensions: &["foguischeme"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4C, 0x49, 0x45, 0x4E, 0x54, 0x5F])],
            },
        }],
    }],
    related_formats: &[],
};
