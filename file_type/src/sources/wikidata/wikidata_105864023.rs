use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864023: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_023,
        source_type: SourceType::Wikidata,
        name: "MidiCo Karaoke",
        extensions: &["mdc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x44, 0x43, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
