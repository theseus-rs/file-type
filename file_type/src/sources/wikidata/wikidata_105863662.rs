use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863662: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_662,
        source_type: SourceType::Wikidata,
        name: "Microsoft Chart for Mac document (v1.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x8C, 0x00, 0x00, 0x13, 0xE8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x01, 0x02, 0x56,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
