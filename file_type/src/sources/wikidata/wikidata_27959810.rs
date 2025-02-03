use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959810: FileFormat = FileFormat {
    id: 27_959_810,
    source_type: SourceType::Wikidata,
    name: "Ableton Live Set",
    extensions: &["als"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
