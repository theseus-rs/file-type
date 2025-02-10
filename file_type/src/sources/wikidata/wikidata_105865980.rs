use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865980: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_980,
        source_type: SourceType::Wikidata,
        name: "Palm Plucker document",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x61, 0x74, 0x61, 0x50, 0x6C, 0x6B, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
