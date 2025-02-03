use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122403904: FileFormat = FileFormat {
    id: 122_403_904,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Constructor Resource File",
    extensions: &["rsr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
