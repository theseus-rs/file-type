use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853906: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_906,
        source_type: SourceType::Wikidata,
        name: "MP3 audio (ID3 v2.x tag)",
        extensions: &["mp3"],
        media_types: &["audio/mpeg3"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
