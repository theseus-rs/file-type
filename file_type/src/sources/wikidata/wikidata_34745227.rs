use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34745227: FileType = FileType {
    file_format: &FileFormat {
        id: 34_745_227,
        source_type: SourceType::Wikidata,
        name: "Spline Font Database",
        extensions: &["sfd"],
        media_types: &["application/vnd.font-fontforge-sfd"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x6C, 0x69, 0x6E, 0x65, 0x46, 0x6F, 0x6E, 0x74, 0x44, 0x42,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
