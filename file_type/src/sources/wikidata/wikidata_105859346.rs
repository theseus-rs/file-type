use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859346: FileFormat = FileFormat {
    id: 105_859_346,
    source_type: SourceType::Wikidata,
    name: "Qubicle Exchange Format",
    extensions: &["qef"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x51, 0x75, 0x62, 0x69, 0x63, 0x6C, 0x65, 0x20, 0x45, 0x78, 0x63, 0x68, 0x61,
                    0x6E, 0x67, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
