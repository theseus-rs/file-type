use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967401: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_401,
        source_type: SourceType::Wikidata,
        name: "XMS-Tracker module",
        extensions: &["xms"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x61, 0x44, 0x6F, 0x4B, 0x61, 0x4E, 0x39, 0x36,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
