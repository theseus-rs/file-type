use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_56827097: FileFormat = FileFormat {
    id: 56_827_097,
    source_type: SourceType::Wikidata,
    name: "Path of Exile GGPK",
    extensions: &["ggpk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
