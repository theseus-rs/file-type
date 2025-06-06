use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855028: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_028,
        source_type: SourceType::Wikidata,
        name: "Dalet Sound format audio (old)",
        extensions: &["snd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xE3, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
