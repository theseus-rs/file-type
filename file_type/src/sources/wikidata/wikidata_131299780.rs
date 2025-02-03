use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131299780: FileFormat = FileFormat {
    id: 131_299_780,
    source_type: SourceType::Wikidata,
    name: "ThingsDB file format",
    extensions: &["ti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
