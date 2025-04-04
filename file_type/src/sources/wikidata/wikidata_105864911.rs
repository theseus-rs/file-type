use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864911: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_911,
        source_type: SourceType::Wikidata,
        name: "Palm Dictionary Reader",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xB0, 0x88, 0x23, 0xAD, 0xB0, 0x88, 0x23, 0xAD, 0xB0, 0x88, 0x23, 0xAD,
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x42, 0x56, 0x6F, 0x6B, 0x42, 0x44, 0x49, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
