use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854267: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_267,
        source_type: SourceType::Wikidata,
        name: "ALICE: The Personal Pascal Program",
        extensions: &["ap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x50, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
