use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866216: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_216,
        source_type: SourceType::Wikidata,
        name: "Photodex ProShow Show file",
        extensions: &["psh"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x68, 0x6F, 0x74, 0x6F, 0x64, 0x65, 0x78, 0x28, 0x52, 0x29, 0x20,
                        0x50, 0x72, 0x6F, 0x53, 0x68, 0x6F, 0x77, 0x28, 0x54, 0x4D, 0x29, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
