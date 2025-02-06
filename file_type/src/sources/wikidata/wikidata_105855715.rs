use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855715: FileFormat = FileFormat {
    id: 105_855_715,
    source_type: SourceType::Wikidata,
    name: "Altium Designer Output Job",
    extensions: &["outjob"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x4F, 0x75, 0x74, 0x70, 0x75, 0x74, 0x4A, 0x6F, 0x62, 0x46, 0x69, 0x6C,
                    0x65, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
