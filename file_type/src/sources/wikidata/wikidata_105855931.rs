use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855931: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_931,
        source_type: SourceType::Wikidata,
        name: "Microsoft separate Debug format",
        extensions: &["dbg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
