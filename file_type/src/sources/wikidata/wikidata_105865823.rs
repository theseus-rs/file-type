use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_823,
        source_type: SourceType::Wikidata,
        name: "Picroma Plasma Graphics",
        extensions: &["plg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                        0x0E, 0x00, 0x00, 0x00, 0x50, 0x6C, 0x61, 0x73, 0x6D, 0x61, 0x47, 0x72,
                        0x61, 0x70, 0x68, 0x69, 0x63, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
