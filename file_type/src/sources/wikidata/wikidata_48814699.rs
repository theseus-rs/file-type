use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48814699: FileFormat = FileFormat {
    id: 48_814_699,
    source_type: SourceType::Wikidata,
    name: "DesignCAD for Windows Drawing",
    extensions: &["dw2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
