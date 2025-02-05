use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850769: FileFormat = FileFormat {
    id: 105_850_769,
    source_type: SourceType::Wikidata,
    name: "Kinemage protein language",
    extensions: &["kin"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x74, 0x65, 0x78, 0x74])],
            },
        }],
    }],
    related_formats: &[],
};
