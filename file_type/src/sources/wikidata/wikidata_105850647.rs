use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850647: FileFormat = FileFormat {
    id: 105_850_647,
    source_type: SourceType::Wikidata,
    name: "Koda Form Designer Form",
    extensions: &["kxf"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
