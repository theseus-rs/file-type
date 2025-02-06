use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837022: FileFormat = FileFormat {
    id: 108_837_022,
    source_type: SourceType::Wikidata,
    name: "Nero Mixed Mode CD Compilation",
    extensions: &["nrm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
