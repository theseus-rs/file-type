use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27960107: FileFormat = FileFormat {
    id: 27_960_107,
    source_type: SourceType::Wikidata,
    name: "Berkeley/IRCAM/Carl Sound Format",
    extensions: &["sf"],
    media_types: &["audio/x-ircam"],
    internal_signatures: &[],
    related_formats: &[],
};
