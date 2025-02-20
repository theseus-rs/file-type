use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43869672: FileType = FileType {
    file_format: &FileFormat {
        id: 43_869_672,
        source_type: SourceType::Wikidata,
        name: "PCX, version 3",
        extensions: &["pcc", "pcx"],
        media_types: &["image/vnd.zbrush.pcx", "image/x-pcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A, 0x05])],
                },
            }],
        }],
        related_formats: &[],
    },
};
