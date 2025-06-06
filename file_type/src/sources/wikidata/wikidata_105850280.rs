use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850280: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_280,
        source_type: SourceType::Wikidata,
        name: "CryptoMite encrypted",
        extensions: &["cryptomite"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x06, 0x4B, 0x72, 0x79, 0x32, 0x30, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
