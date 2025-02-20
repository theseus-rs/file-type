use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854568: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_568,
        source_type: SourceType::Wikidata,
        name: "Altirra save state",
        extensions: &["altstate"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x6C, 0x74, 0x53, 0x61, 0x76, 0x65, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
