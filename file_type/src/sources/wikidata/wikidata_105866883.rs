use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_883,
        source_type: SourceType::Wikidata,
        name: "JBuilder Beans Descriptor",
        extensions: &["pme"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x42, 0x65, 0x61, 0x6E, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
                        0x74, 0x6F, 0x72, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
