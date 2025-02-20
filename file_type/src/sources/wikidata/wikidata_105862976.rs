use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862976: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_976,
        source_type: SourceType::Wikidata,
        name: "Mystic BBS install package",
        extensions: &["mys"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x04, 0x4D, 0x59, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
