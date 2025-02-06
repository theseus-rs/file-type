use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34743987: FileFormat = FileFormat {
    id: 34_743_987,
    source_type: SourceType::Wikidata,
    name: "Spark",
    extensions: &["spk"],
    media_types: &["archive"],
    signatures: &[],
    related_formats: &[],
};
