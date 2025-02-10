use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853382: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_382,
        source_type: SourceType::Wikidata,
        name: "Microsoft Speech Data",
        extensions: &["spd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x61, 0x74, 0x61, 0x00, 0x00, 0x01, 0x00, 0x33, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
