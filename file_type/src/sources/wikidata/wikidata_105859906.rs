use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859906: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_906,
        source_type: SourceType::Wikidata,
        name: "Music-X Voices",
        extensions: &["voices"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x58, 0x46, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
