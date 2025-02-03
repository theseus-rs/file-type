use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27925705: FileFormat = FileFormat {
    id: 27_925_705,
    source_type: SourceType::Wikidata,
    name: "DTED Readme file",
    extensions: &["me"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
