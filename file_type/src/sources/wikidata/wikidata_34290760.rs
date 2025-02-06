use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34290760: FileFormat = FileFormat {
    id: 34_290_760,
    source_type: SourceType::Wikidata,
    name: "Statistical Package for the Social Sciences syntax file",
    extensions: &["sps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
