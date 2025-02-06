use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049507: FileFormat = FileFormat {
    id: 28_049_507,
    source_type: SourceType::Wikidata,
    name: "NEOchrome",
    extensions: &["neo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
