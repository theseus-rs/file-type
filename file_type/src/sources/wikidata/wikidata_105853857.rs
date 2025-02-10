use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853857: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_857,
        source_type: SourceType::Wikidata,
        name: "ArcSoft UI",
        extensions: &["aui"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x55, 0x49, 0x00, 0x00, 0x01, 0x00, 0x00, 0x41, 0x72, 0x63, 0x73,
                        0x6F, 0x66, 0x74, 0x20, 0x55, 0x49, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
