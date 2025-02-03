use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110238400: FileFormat = FileFormat {
    id: 110_238_400,
    source_type: SourceType::Wikidata,
    name: "Screenwriter 6 file format",
    extensions: &["mmsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
