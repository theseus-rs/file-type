use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7265434: FileFormat = FileFormat {
    id: 7_265_434,
    source_type: SourceType::Wikidata,
    name: "Quicken Financial Exchange",
    extensions: &["qfx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
