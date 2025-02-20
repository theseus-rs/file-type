use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853161: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_161,
        source_type: SourceType::Wikidata,
        name: "Yamaha Midimonitor/BULK Manager Symbols",
        extensions: &["sbl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x53, 0x42, 0x4C, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
