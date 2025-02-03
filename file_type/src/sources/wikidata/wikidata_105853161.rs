use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853161: FileFormat = FileFormat {
    id: 105_853_161,
    source_type: SourceType::Wikidata,
    name: "Yamaha Midimonitor/BULK Manager Symbols",
    extensions: &["sbl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x53, 0x42, 0x4C, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
