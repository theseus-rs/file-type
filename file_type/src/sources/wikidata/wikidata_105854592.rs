use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854592: FileFormat = FileFormat {
    id: 105_854_592,
    source_type: SourceType::Wikidata,
    name: "Opera Hotlist (v2.0) / bookmark",
    extensions: &["adr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x70, 0x65, 0x72, 0x61, 0x20, 0x48, 0x6F, 0x74, 0x6C, 0x69, 0x73, 0x74,
                    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
