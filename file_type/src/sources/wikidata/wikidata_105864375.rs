use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864375: FileFormat = FileFormat {
    id: 105_864_375,
    source_type: SourceType::Wikidata,
    name: "ParaView state",
    extensions: &["pvsm"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x50, 0x61, 0x72, 0x61, 0x56, 0x69, 0x65, 0x77, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
