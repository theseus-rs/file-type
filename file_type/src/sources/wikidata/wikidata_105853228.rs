use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_228,
        source_type: SourceType::Wikidata,
        name: "The Music Studio Song (Atari ST)",
        extensions: &["sng"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xCD, 0x4D, 0x73, 0x74, 0x75, 0x64, 0x69, 0x6F, 0xCD, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
