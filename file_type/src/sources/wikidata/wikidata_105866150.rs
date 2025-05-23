use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866150: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_150,
        source_type: SourceType::Wikidata,
        name: "Palm JFile Pro database",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x66, 0x44, 0x62, 0x4A, 0x46, 0x69, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
