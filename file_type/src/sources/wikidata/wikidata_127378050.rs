use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127378050: FileFormat = FileFormat {
    id: 127_378_050,
    source_type: SourceType::Wikidata,
    name: "Pyrex Source Code File",
    extensions: &["pyx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
