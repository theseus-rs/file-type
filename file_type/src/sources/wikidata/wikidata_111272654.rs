use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272654: FileFormat = FileFormat {
    id: 111_272_654,
    source_type: SourceType::Wikidata,
    name: "ESPS audio file",
    extensions: &["esps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
