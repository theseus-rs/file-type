use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867428: FileFormat = FileFormat {
    id: 105_867_428,
    source_type: SourceType::Wikidata,
    name: "Apple Interface Builder NIB archive (binary)",
    extensions: &["nib"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x49, 0x42, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
