use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853746: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_746,
        source_type: SourceType::Wikidata,
        name: "AND XSynth module",
        extensions: &["amx"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4D, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
