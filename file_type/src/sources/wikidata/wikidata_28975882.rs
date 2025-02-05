use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975882: FileFormat = FileFormat {
    id: 28_975_882,
    source_type: SourceType::Wikidata,
    name: "SOLIDWORKS Assembly",
    extensions: &["sldasm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
