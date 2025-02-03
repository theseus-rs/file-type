use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121092987: FileFormat = FileFormat {
    id: 121_092_987,
    source_type: SourceType::Wikidata,
    name: "Punch! 3D Object",
    extensions: &["pob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
