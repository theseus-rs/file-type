use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850485: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_485,
        source_type: SourceType::Wikidata,
        name: "CIRCAD data (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
