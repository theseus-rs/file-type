use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_1143961: FileType = FileType {
    file_format: &FileFormat {
        id: 1_143_961,
        source_type: SourceType::Wikidata,
        name: "JBIG2",
        extensions: &["jb2", "jbig2"],
        media_types: &["image/x-jbig2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x97, 0x4A, 0x42, 0x32, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
