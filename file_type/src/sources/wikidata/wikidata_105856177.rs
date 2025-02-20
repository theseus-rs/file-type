use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856177: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_177,
        source_type: SourceType::Wikidata,
        name: "DBC Communication Database for CAN",
        extensions: &["dbc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20, 0x22, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
