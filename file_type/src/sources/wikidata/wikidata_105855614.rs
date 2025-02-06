use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855614: FileFormat = FileFormat {
    id: 105_855_614,
    source_type: SourceType::Wikidata,
    name: "OpenShot Project",
    extensions: &["osp"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x69, 0x6F, 0x70, 0x65, 0x6E, 0x73, 0x68, 0x6F, 0x74, 0x2E, 0x63, 0x6C,
                    0x61, 0x73, 0x73, 0x65, 0x73, 0x2E, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
