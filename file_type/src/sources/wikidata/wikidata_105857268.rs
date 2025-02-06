use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857268: FileFormat = FileFormat {
    id: 105_857_268,
    source_type: SourceType::Wikidata,
    name: "PC-Bibliothek Hierarchy Compressed",
    extensions: &["hic"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x50, 0x43,
                    0x2D, 0x42, 0x69, 0x62, 0x6C, 0x69, 0x6F, 0x74, 0x68, 0x65, 0x6B, 0x20, 0x48,
                    0x69, 0x65, 0x72, 0x61, 0x72, 0x63, 0x68, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
