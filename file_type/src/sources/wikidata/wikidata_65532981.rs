use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65532981: FileFormat = FileFormat {
    id: 65_532_981,
    source_type: SourceType::Wikidata,
    name: "Cookbook file format",
    extensions: &["mc2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
