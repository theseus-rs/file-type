use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859804: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_804,
        source_type: SourceType::Wikidata,
        name: "Eyemail video",
        extensions: &["eye"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x65, 0x79, 0x65, 0x6D, 0x61, 0x69, 0x6C, 0x76, 0x69, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
