use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27925714: FileFormat = FileFormat {
    id: 27_925_714,
    source_type: SourceType::Wikidata,
    name: "DTED Level 1 Gazetteer Hash file",
    extensions: &["hsh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
