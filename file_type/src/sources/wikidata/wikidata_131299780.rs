use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131299780: FileFormat = FileFormat {
    id: 131_299_780,
    source_type: SourceType::Wikidata,
    name: "ThingsDB file format",
    extensions: &["ti"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
