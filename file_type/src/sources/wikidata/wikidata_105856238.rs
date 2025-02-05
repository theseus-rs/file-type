use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856238: FileFormat = FileFormat {
    id: 105_856_238,
    source_type: SourceType::Wikidata,
    name: "Bloodshed Dev-C++ project",
    extensions: &["dev"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
