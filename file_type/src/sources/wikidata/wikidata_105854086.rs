use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854086: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_086,
        source_type: SourceType::Wikidata,
        name: "Jet-VoiceMail audio data",
        extensions: &["cvf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x57, 0x4F, 0x4E, 0x56, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
