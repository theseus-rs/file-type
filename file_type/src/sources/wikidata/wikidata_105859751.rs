use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859751: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_751,
        source_type: SourceType::Wikidata,
        name: "Microsoft Professional ISAM database",
        extensions: &["mdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
