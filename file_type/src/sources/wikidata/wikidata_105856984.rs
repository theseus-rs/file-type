use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856984: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_984,
        source_type: SourceType::Wikidata,
        name: "Spectrum Global Tracker chiptune",
        extensions: &["gtr"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x54, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
