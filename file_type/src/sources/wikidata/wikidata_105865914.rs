use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865914: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_914,
        source_type: SourceType::Wikidata,
        name: "Palm Query Application",
        extensions: &["pqa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x71, 0x61, 0x20, 0x63, 0x6C, 0x70, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
