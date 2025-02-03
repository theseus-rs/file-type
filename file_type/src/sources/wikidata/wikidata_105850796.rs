use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850796: FileFormat = FileFormat {
    id: 105_850_796,
    source_type: SourceType::Wikidata,
    name: "Klystrack chiptune",
    extensions: &["kt"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x79, 0x64, 0x21, 0x73, 0x6F, 0x6E, 0x67,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
