use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858658: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_658,
        source_type: SourceType::Wikidata,
        name: "PABX Background bitmap",
        extensions: &["pix"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x58, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
