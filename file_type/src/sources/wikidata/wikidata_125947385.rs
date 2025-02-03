use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125947385: FileFormat = FileFormat {
    id: 125_947_385,
    source_type: SourceType::Wikidata,
    name: "Finale Notation File 2014+",
    extensions: &["musx"],
    media_types: &["application/vnd.makemusic.notation"],
    internal_signatures: &[],
    related_formats: &[],
};
