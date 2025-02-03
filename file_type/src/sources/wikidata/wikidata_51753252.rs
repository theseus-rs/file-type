use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51753252: FileFormat = FileFormat {
    id: 51_753_252,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Compiled Shape/Font File",
    extensions: &["shx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
