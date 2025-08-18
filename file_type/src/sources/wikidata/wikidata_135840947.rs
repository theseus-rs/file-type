use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_135840947: FileType = FileType {
    file_format: &FileFormat {
        id: 135_840_947,
        source_type: SourceType::Wikidata,
        name: "SPC Runtime Presentation",
        extensions: &["ply"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x53, 0x50, 0x43, 0x52]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
