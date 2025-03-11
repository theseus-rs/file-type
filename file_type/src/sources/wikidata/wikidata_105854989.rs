use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854989: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_989,
        source_type: SourceType::Wikidata,
        name: "Acorn ArcFS Archive (var)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x1A, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
