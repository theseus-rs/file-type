use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849304: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_304,
        source_type: SourceType::Wikidata,
        name: "YouiDraw Drawing",
        extensions: &["ydr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x79, 0x64, 0x72, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
