use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66439341: FileFormat = FileFormat {
    id: 66_439_341,
    source_type: SourceType::Wikidata,
    name: "Volkswriter file format",
    extensions: &["vw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
