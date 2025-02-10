use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865884: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_884,
        source_type: SourceType::Wikidata,
        name: "Spectrum Pro Sound Maker chiptune",
        extensions: &["psm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x73, 0x6D, 0x31, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
