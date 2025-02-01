use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850922: FileFormat = FileFormat {
    id: 105_850_922,
    puid: "wikidata/105850922",
    name: "Oracle Trace Metadata",
    extensions: &["trm"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x72, 0x61, 0x63, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
