use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127378050: FileFormat = FileFormat {
    id: 127_378_050,
    source_type: SourceType::Wikidata,
    name: "Pyrex Source Code File",
    extensions: &["pyx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
