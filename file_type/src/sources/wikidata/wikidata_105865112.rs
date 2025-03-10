use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865112: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_112,
        source_type: SourceType::Wikidata,
        name: "ProvideX data (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x50, 0x56, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
