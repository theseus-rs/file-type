use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856401: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_401,
        source_type: SourceType::Wikidata,
        name: "Wink presentation",
        extensions: &["wnk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x50, 0x4B, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
