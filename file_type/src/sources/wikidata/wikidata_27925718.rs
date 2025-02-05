use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925718: FileFormat = FileFormat {
    id: 27_925_718,
    source_type: SourceType::Wikidata,
    name: "DTED Level 1 Gazetteer Key file",
    extensions: &["key"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
