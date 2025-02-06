use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864562: FileFormat = FileFormat {
    id: 105_864_562,
    source_type: SourceType::Wikidata,
    name: "PostScript Type 1 Font (v1.1)",
    extensions: &["pfa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x21, 0x46, 0x6F, 0x6E, 0x74, 0x54, 0x79, 0x70, 0x65, 0x31, 0x2D, 0x31,
                    0x2E, 0x31, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
