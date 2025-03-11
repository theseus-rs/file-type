use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853813: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_813,
        source_type: SourceType::Wikidata,
        name: "KBOOM11 compressed",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA8, 0x4D, 0x50, 0xA8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
