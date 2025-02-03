use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110984254: FileFormat = FileFormat {
    id: 110_984_254,
    source_type: SourceType::Wikidata,
    name: "Corel VideoStudio Project File",
    extensions: &["vsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
