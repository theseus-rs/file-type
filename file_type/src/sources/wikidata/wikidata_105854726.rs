use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854726: FileFormat = FileFormat {
    id: 105_854_726,
    source_type: SourceType::Wikidata,
    name: "openEHR Archetype Definition Language",
    extensions: &["adl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x61, 0x72, 0x63, 0x68, 0x65, 0x74, 0x79, 0x70, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
