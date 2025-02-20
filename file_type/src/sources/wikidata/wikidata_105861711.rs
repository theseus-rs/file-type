use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861711: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_711,
        source_type: SourceType::Wikidata,
        name: "McGrath Information Solution metadata",
        extensions: &["mis"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6E, 0x74, 0x65, 0x6E, 0x74, 0x2D, 0x73, 0x74, 0x61, 0x72,
                        0x74, 0x20, 0x20, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
