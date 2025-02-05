use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856619: FileFormat = FileFormat {
    id: 105_856_619,
    source_type: SourceType::Wikidata,
    name: "Windows Metafile (old Win 3.x format)",
    extensions: &["wmf"],
    media_types: &["image/x-wmf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x00, 0x09, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
