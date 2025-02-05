use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837072: FileFormat = FileFormat {
    id: 108_837_072,
    source_type: SourceType::Wikidata,
    name: "Nero HFS-CD Compilation",
    extensions: &["nhf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
