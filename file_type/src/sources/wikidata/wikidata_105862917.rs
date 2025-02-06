use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862917: FileFormat = FileFormat {
    id: 105_862_917,
    source_type: SourceType::Wikidata,
    name: "NeXtMidas Macro",
    extensions: &["mm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x74, 0x61, 0x72, 0x74, 0x6D, 0x61, 0x63, 0x72, 0x6F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
