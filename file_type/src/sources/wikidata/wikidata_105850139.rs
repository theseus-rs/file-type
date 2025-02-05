use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850139: FileFormat = FileFormat {
    id: 105_850_139,
    source_type: SourceType::Wikidata,
    name: "Adobe Photoshop Custom Shape",
    extensions: &["csh"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x75, 0x73, 0x68, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
