use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967224: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_224,
        source_type: SourceType::Wikidata,
        name: "Ultra Tracker",
        extensions: &["ult"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x41, 0x53, 0x5F, 0x55, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x5F, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
