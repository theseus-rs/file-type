use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121544667: FileFormat = FileFormat {
    id: 121_544_667,
    source_type: SourceType::Wikidata,
    name: "At Home 2009 Tax Return File",
    extensions: &["t09"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
