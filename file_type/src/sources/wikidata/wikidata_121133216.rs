use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121133216: FileFormat = FileFormat {
    id: 121_133_216,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows BitMap",
    extensions: &["bmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
