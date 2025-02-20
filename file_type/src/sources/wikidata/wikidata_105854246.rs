use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854246: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_246,
        source_type: SourceType::Wikidata,
        name: "AES Crypt encrypted",
        extensions: &["aes"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x45, 0x53, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
