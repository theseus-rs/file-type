use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858883: FileFormat = FileFormat {
    id: 105_858_883,
    source_type: SourceType::Wikidata,
    name: "Dore Raster bitmap (with rem)",
    extensions: &["dore", "img"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
