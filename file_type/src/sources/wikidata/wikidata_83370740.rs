use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83370740: FileFormat = FileFormat {
    id: 83_370_740,
    source_type: SourceType::Wikidata,
    name: "Audio Visual Research",
    extensions: &["avr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
