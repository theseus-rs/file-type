use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_70892998: FileFormat = FileFormat {
    id: 70_892_998,
    source_type: SourceType::Wikidata,
    name: "Altera Graphic Design File",
    extensions: &["gdf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x44, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
