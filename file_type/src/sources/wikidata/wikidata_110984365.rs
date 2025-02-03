use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110984365: FileFormat = FileFormat {
    id: 110_984_365,
    source_type: SourceType::Wikidata,
    name: "Corel VideoStudio HTML5 Project File",
    extensions: &["vsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
