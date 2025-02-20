use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865466: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_466,
        source_type: SourceType::Wikidata,
        name: "Palm Tides",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x64, 0x61, 0x74, 0x54, 0x69, 0x64, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
