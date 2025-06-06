use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864990: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_990,
        source_type: SourceType::Wikidata,
        name: "Palm TealMeal",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x44, 0x61, 0x74, 0x61, 0x54, 0x6C, 0x4D, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
