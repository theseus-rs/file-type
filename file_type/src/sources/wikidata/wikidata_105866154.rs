use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866154: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_154,
        source_type: SourceType::Wikidata,
        name: "Palm Bible+ document",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x62, 0x69, 0x62, 0x6C, 0x50, 0x50, 0x42, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
