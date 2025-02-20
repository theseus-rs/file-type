use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862352: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_352,
        source_type: SourceType::Wikidata,
        name: "Mobiclip for Nintendo CTR",
        extensions: &["moflex"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x32, 0xAA, 0xAB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
