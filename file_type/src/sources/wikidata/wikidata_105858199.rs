use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858199: FileFormat = FileFormat {
    id: 105_858_199,
    source_type: SourceType::Wikidata,
    name: "Sony Playstation Executable",
    extensions: &["exe"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x2D, 0x58, 0x20, 0x45, 0x58, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
