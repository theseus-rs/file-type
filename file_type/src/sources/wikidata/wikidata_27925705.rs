use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925705: FileFormat = FileFormat {
    id: 27_925_705,
    source_type: SourceType::Wikidata,
    name: "DTED Readme file",
    extensions: &["me"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
