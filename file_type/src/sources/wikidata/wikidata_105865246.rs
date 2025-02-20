use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865246: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_246,
        source_type: SourceType::Wikidata,
        name: "Palm TealMovie video+audio",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x4D, 0x76, 0x69, 0x65, 0x54, 0x6C, 0x4D, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
