use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865033: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_033,
        source_type: SourceType::Wikidata,
        name: "GRUB2 font",
        extensions: &["pf2"],
        media_types: &["application/x-font-pf2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x49, 0x4C, 0x45, 0x00, 0x00, 0x00, 0x04, 0x50, 0x46, 0x46, 0x32,
                        0x4E, 0x41, 0x4D, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
