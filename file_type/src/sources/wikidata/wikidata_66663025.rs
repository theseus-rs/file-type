use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663025: FileFormat = FileFormat {
    id: 66_663_025,
    source_type: SourceType::Wikidata,
    name: "Lotus Freelance SmartMaster Content",
    extensions: &["smc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
