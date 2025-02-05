use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127378389: FileFormat = FileFormat {
    id: 127_378_389,
    source_type: SourceType::Wikidata,
    name: "Genie source code file",
    extensions: &["gs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
