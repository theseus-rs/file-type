use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857161: FileFormat = FileFormat {
    id: 105_857_161,
    source_type: SourceType::Wikidata,
    name: "HammerHead Rhythm Station pattern",
    extensions: &["hh"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x34, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
                    0x20, 0x48, 0x61, 0x6D, 0x6D, 0x65, 0x72, 0x48, 0x65, 0x61, 0x64, 0x20, 0x52,
                    0x68, 0x79, 0x74, 0x68, 0x6D, 0x20, 0x53, 0x74, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                    0x20, 0x62, 0x79, 0x20, 0x42, 0x72, 0x61, 0x6D, 0x20, 0x42, 0x6F, 0x73, 0x2E,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
