use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864800: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_800,
        source_type: SourceType::Wikidata,
        name: "PC-File database header",
        extensions: &["dbh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x42, 0x48, 0x00, 0x6A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
