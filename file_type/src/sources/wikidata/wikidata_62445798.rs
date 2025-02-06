use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62445798: FileFormat = FileFormat {
    id: 62_445_798,
    source_type: SourceType::Wikidata,
    name: "OWL XML Serialization",
    extensions: &["owx"],
    media_types: &["application/owl+xml"],
    signatures: &[],
    related_formats: &[],
};
