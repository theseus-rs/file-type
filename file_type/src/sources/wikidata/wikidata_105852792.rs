use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852792: FileFormat = FileFormat {
    id: 105_852_792,
    source_type: SourceType::Wikidata,
    name: "Compiled AppleScript script",
    extensions: &["scpt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x61, 0x73, 0x64, 0x55, 0x41, 0x53, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
