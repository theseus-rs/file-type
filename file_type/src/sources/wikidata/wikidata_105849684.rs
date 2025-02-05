use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849684: FileFormat = FileFormat {
    id: 105_849_684,
    source_type: SourceType::Wikidata,
    name: "Csound unified file format",
    extensions: &["csd"],
    media_types: &["audio/csound"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x73, 0x6F, 0x75, 0x6E, 0x64, 0x53, 0x79, 0x6E, 0x74, 0x68, 0x65,
                    0x73, 0x69, 0x7A, 0x65, 0x72, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
