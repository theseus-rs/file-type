use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2375766: FileFormat = FileFormat {
    id: 2_375_766,
    source_type: SourceType::Wikidata,
    name: "Synchronized Accessible Media Interchange",
    extensions: &["sami", "smi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
