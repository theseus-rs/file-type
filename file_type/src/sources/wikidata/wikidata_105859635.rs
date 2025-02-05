use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859635: FileFormat = FileFormat {
    id: 105_859_635,
    source_type: SourceType::Wikidata,
    name: "VirtuaNES movie capture",
    extensions: &["vmv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x4E, 0x45, 0x53, 0x20, 0x4D, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
