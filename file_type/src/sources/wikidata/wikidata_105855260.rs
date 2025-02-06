use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855260: FileFormat = FileFormat {
    id: 105_855_260,
    source_type: SourceType::Wikidata,
    name: "FOnline Engine GUI",
    extensions: &["fogui"],
    media_types: &["text/json"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEF, 0xBB, 0xBF, 0x7B])],
            },
        }],
    }],
    related_formats: &[],
};
