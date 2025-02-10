use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_245,
        source_type: SourceType::Wikidata,
        name: "Microsoft Chat Background Graphic",
        extensions: &["bgb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x81, 0x81, 0x03, 0x00, 0x02, 0x00, 0x03, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
