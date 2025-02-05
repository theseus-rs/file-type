use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48810278: FileFormat = FileFormat {
    id: 48_810_278,
    source_type: SourceType::Wikidata,
    name: "DesignCAD Drawing",
    extensions: &["dc", "dc2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
