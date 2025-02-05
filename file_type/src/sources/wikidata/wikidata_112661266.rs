use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661266: FileFormat = FileFormat {
    id: 112_661_266,
    source_type: SourceType::Wikidata,
    name: "Lightscape Preparation File",
    extensions: &["lp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
