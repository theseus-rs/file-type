use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121092987: FileFormat = FileFormat {
    id: 121_092_987,
    source_type: SourceType::Wikidata,
    name: "Punch! 3D Object",
    extensions: &["pob"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
