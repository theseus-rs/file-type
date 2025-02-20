use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864658: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_658,
        source_type: SourceType::Wikidata,
        name: "PowerBackup Job",
        extensions: &["pbj"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x6D, 0x61, 0x67,
                        0x69, 0x63, 0x3D, 0x22, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
