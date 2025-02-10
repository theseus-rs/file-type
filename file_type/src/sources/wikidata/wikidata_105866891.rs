use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866891: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_891,
        source_type: SourceType::Wikidata,
        name: "Palm JFile database",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x62, 0x44, 0x62, 0x4A, 0x42, 0x61, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
