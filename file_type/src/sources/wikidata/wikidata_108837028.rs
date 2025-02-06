use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837028: FileFormat = FileFormat {
    id: 108_837_028,
    source_type: SourceType::Wikidata,
    name: "Nero CD EXTRA Compilation",
    extensions: &["nrm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
