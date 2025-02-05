use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51753252: FileFormat = FileFormat {
    id: 51_753_252,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Compiled Shape/Font File",
    extensions: &["shx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
