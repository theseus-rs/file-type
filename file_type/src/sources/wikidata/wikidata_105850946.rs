use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850946: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_946,
        source_type: SourceType::Wikidata,
        name: "TurboFM Compiler chiptune",
        extensions: &["tfc"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x46, 0x4D, 0x63, 0x6F, 0x6D, 0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
