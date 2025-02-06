use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851091: FileFormat = FileFormat {
    id: 105_851_091,
    source_type: SourceType::Wikidata,
    name: "TermBase eXchange Format (UTF-8)",
    extensions: &["tbx"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73,
                    0x69, 0x6F, 0x6E, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
