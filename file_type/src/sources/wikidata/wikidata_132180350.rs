use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132180350: FileType = FileType {
    file_format: &FileFormat {
        id: 132_180_350,
        source_type: SourceType::Wikidata,
        name: "Content Addressable Archive",
        extensions: &["car"],
        media_types: &["application/vnd.ipld.car"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0xA1, 0x67, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
