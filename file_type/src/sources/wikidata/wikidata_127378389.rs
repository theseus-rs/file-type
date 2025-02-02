use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127378389: FileFormat = FileFormat {
    id: 127_378_389,
    source_type: SourceType::Wikidata,
    name: "Genie source code file",
    extensions: &["gs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
