use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867073: FileFormat = FileFormat {
    id: 105_867_073,
    source_type: SourceType::Wikidata,
    name: "NITF National Imagery Transmission Format image (v1.x)",
    extensions: &["nitf", "ntf"],
    media_types: &["application/vnd.nitf"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4E, 0x49, 0x54, 0x46, 0x30, 0x31, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
