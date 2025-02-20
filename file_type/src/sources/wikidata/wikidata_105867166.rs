use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867166: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_166,
        source_type: SourceType::Wikidata,
        name: "NetStumbler NS1 log",
        extensions: &["ns1"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x65, 0x74, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
