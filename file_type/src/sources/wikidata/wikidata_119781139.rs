use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119781139: FileFormat = FileFormat {
    id: 119_781_139,
    source_type: SourceType::Wikidata,
    name: "Street Atlas USA File",
    extensions: &["saf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
