use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121133216: FileFormat = FileFormat {
    id: 121_133_216,
    source_type: SourceType::Wikidata,
    name: "TurboCAD for Windows BitMap",
    extensions: &["bmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
