use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852847: FileFormat = FileFormat {
    id: 105_852_847,
    source_type: SourceType::Wikidata,
    name: "blueMSX Shortcuts",
    extensions: &["shortcuts"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x68, 0x6F, 0x72, 0x74, 0x63, 0x75, 0x74, 0x73, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
