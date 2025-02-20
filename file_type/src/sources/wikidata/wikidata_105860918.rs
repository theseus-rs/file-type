use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860918: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_918,
        source_type: SourceType::Wikidata,
        name: "BlackBerry resource",
        extensions: &["rrh"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
