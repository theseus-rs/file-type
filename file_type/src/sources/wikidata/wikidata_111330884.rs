use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111330884: FileFormat = FileFormat {
    id: 111_330_884,
    source_type: SourceType::Wikidata,
    name: "Musifile MPEG Layer II audio stream",
    extensions: &["mus"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
