use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112660704: FileFormat = FileFormat {
    id: 112_660_704,
    source_type: SourceType::Wikidata,
    name: "Portfolio File",
    extensions: &["bfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
