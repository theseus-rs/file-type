use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865379: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_379,
        source_type: SourceType::Wikidata,
        name: "Microsoft Program DataBase (v7)",
        extensions: &["pdb"],
        media_types: &["application/x-ms-pdb"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x43, 0x2F,
                        0x43, 0x2B, 0x2B, 0x20, 0x4D, 0x53, 0x46, 0x20, 0x37, 0x2E, 0x30, 0x30,
                        0x0D, 0x0A, 0x1A, 0x44, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
