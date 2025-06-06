use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851765: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_765,
        source_type: SourceType::Wikidata,
        name: "Lotus Metro Screen Driver",
        extensions: &["sdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x04, 0x00, 0x44, 0x69, 0x73, 0x70, 0x6C, 0x61, 0x79, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
