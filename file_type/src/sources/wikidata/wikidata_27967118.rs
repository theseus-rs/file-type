use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967118: FileFormat = FileFormat {
    id: 27_967_118,
    source_type: SourceType::Wikidata,
    name: "Beepola song",
    extensions: &["bbsong"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x42, 0x53, 0x4F, 0x4E, 0x47, 0x00, 0x30, 0x30, 0x30, 0x31, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
