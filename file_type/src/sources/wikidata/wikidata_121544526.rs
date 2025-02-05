use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121544526: FileFormat = FileFormat {
    id: 121_544_526,
    source_type: SourceType::Wikidata,
    name: "At Home 2010 Tax Return File",
    extensions: &["t10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
