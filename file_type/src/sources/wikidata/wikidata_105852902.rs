use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852902: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_902,
        source_type: SourceType::Wikidata,
        name: "GoatTracker chiptune",
        extensions: &["sng"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x54, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
